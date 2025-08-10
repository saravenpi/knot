use crate::auth::{AuthenticatedUser, DbPool};
use crate::models::{PackageResponse, PublishPackageRequest};
use crate::validation::{
    validate_package_name, validate_package_team_consistency, validate_version,
};
use chrono::Utc;
use hex;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;

#[derive(FromForm)]
pub struct PackageUpload<'r> {
    pub metadata: String, // JSON string of PublishPackageRequest
    pub file: TempFile<'r>,
}

#[post("/packages/publish", data = "<upload>")]
pub async fn publish_package(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
    mut upload: Form<PackageUpload<'_>>,
) -> Result<Json<PackageResponse>, Status> {
    // Parse metadata JSON
    let request: PublishPackageRequest = match serde_json::from_str(&upload.metadata) {
        Ok(req) => req,
        Err(_) => return Err(Status::BadRequest),
    };

    // Validate input
    if let Err(_) = request.validate() {
        return Err(Status::BadRequest);
    }

    // Generate proper package name with @ prefix
    let package_name = if let Some(team_name) = &request.team_name {
        format!("@{}/{}", team_name, request.name)
    } else {
        format!("@{}", request.name)
    };

    // Validate package name format (must start with @)
    if !validate_package_name(&package_name) {
        return Err(Status::BadRequest);
    }

    // Validate version format (semantic versioning)
    if !validate_version(&request.version) {
        return Err(Status::BadRequest);
    }

    // Validate package-team consistency
    if !validate_package_team_consistency(&package_name, request.team_name.as_deref()) {
        return Err(Status::BadRequest);
    }

    // Check file size (limit to 100MB)
    let file_size = upload.file.len() as i64;
    if file_size > 100 * 1024 * 1024 {
        return Err(Status::PayloadTooLarge);
    }

    // Get team if specified
    let team_id = if let Some(team_name) = &request.team_name {
        // Check if user is a member of the team
        let team_result = sqlx::query!(
            r#"
            SELECT t.id
            FROM teams t
            JOIN team_members tm ON t.id = tm.team_id
            WHERE t.name = $1 AND tm.user_id = $2
            "#,
            team_name,
            auth_user.user_id
        )
        .fetch_optional(&mut **db)
        .await;

        match team_result {
            Ok(Some(row)) => Some(row.id),
            Ok(None) => return Err(Status::Forbidden), // User not in team or team doesn't exist
            Err(_) => return Err(Status::InternalServerError),
        }
    } else {
        None
    };

    // Check if package already exists with same name/version
    let existing_package = sqlx::query!(
        "SELECT id FROM packages WHERE name = $1 AND version = $2",
        package_name,
        request.version
    )
    .fetch_optional(&mut **db)
    .await;

    match existing_package {
        Ok(Some(_)) => return Err(Status::Conflict),
        Ok(None) => {}
        Err(_) => return Err(Status::InternalServerError),
    }

    // Save file to storage
    let package_id = Uuid::new_v4();
    let safe_name = package_name.replace("/", "-").replace("@", "");
    let file_name = format!("{}-{}-{}.tar.gz", safe_name, request.version, package_id);

    let storage_path = format!("storage/packages/{}", file_name);
    let download_url = format!("/api/packages/download/{}", package_id);

    // Create storage directory if it doesn't exist
    if let Err(_) = std::fs::create_dir_all("storage/packages") {
        return Err(Status::InternalServerError);
    }

    // Read file content for checksum calculation
    let file_content = tokio::fs::read(upload.file.path().ok_or(Status::InternalServerError)?)
        .await
        .map_err(|_| Status::InternalServerError)?;

    // Calculate SHA256 checksum
    let mut hasher = Sha256::new();
    hasher.update(&file_content);
    let checksum_sha256 = hex::encode(hasher.finalize());

    // Save file to storage
    std::fs::write(&storage_path, &file_content).map_err(|_| Status::InternalServerError)?;

    let now = Utc::now();

    // Create package record
    let result = sqlx::query_as!(
        Package,
        r#"
        INSERT INTO packages (id, name, version, description, owner_id, team_id, download_url, file_path, file_size, checksum_sha256, downloads_count, published_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING id, name, version, description, owner_id, team_id, download_url, file_path, file_size, checksum_sha256, downloads_count, published_at, updated_at
        "#,
        package_id,
        package_name,
        request.version,
        request.description,
        auth_user.user_id,
        team_id,
        download_url,
        storage_path,
        file_size,
        checksum_sha256,
        0i64,
        now,
        now
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(package) => {
            // Get owner info
            let owner = sqlx::query_as!(
                UserProfile,
                "SELECT id, username, email, created_at FROM users WHERE id = $1",
                auth_user.user_id
            )
            .fetch_one(&mut **db)
            .await
            .map_err(|_| Status::InternalServerError)?;

            // Get team info if applicable
            let team = if let Some(tid) = team_id {
                sqlx::query_as!(
                    Team,
                    "SELECT id, name, description, owner_id, created_at, updated_at FROM teams WHERE id = $1",
                    tid
                )
                .fetch_optional(&mut **db)
                .await
                .map_err(|_| Status::InternalServerError)?
            } else {
                None
            };

            // Handle tags if provided
            if let Some(tags) = &request.tags {
                for tag in tags {
                    // Validate tag format
                    if !tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
                        || tag.starts_with('-')
                        || tag.ends_with('-')
                        || tag.len() > 50
                    {
                        continue; // Skip invalid tags
                    }

                    let tag_lower = tag.to_lowercase();
                    let _ = sqlx::query!(
                        "INSERT INTO package_tags (package_id, tag) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                        package.id,
                        tag_lower
                    )
                    .execute(&mut **db)
                    .await;
                }
            }

            let response = PackageResponse {
                id: package.id,
                name: package.name,
                version: package.version,
                description: package.description,
                owner,
                team,
                download_url: package.download_url,
                file_size: package.file_size,
                published_at: package.published_at,
            };

            Ok(Json(response))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/packages")]
pub async fn list_packages(mut db: DbPool) -> Result<Json<Vec<serde_json::Value>>, Status> {
    let packages_result = sqlx::query!(
        r#"
        SELECT 
            p.id, p.name, p.version, p.description, p.file_size, p.published_at,
            u.id as owner_id, u.username as owner_username, u.email as owner_email, u.created_at as owner_created_at,
            t.id as team_id, t.name as team_name, t.description as team_description, t.owner_id as team_owner_id, t.created_at as team_created_at, t.updated_at as team_updated_at
        FROM packages p
        JOIN users u ON p.owner_id = u.id
        LEFT JOIN teams t ON p.team_id = t.id
        ORDER BY p.published_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&mut **db)
    .await;

    match packages_result {
        Ok(rows) => {
            let packages: Vec<serde_json::Value> = rows
                .into_iter()
                .map(|row| {
                    let team = if let Some(team_id) = row.team_id {
                        Some(serde_json::json!({
                            "id": team_id,
                            "name": row.team_name,
                            "description": row.team_description,
                            "owner_id": row.team_owner_id,
                            "created_at": row.team_created_at,
                            "updated_at": row.team_updated_at
                        }))
                    } else {
                        None
                    };

                    serde_json::json!({
                        "id": row.id,
                        "name": row.name,
                        "version": row.version,
                        "description": row.description,
                        "file_size": row.file_size,
                        "published_at": row.published_at,
                        "owner": {
                            "id": row.owner_id,
                            "username": row.owner_username,
                            "email": row.owner_email,
                            "created_at": row.owner_created_at
                        },
                        "team": team
                    })
                })
                .collect();
            Ok(Json(packages))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/packages/<name>")]
pub async fn get_package_versions(
    mut db: DbPool,
    name: &str,
) -> Result<Json<Vec<serde_json::Value>>, Status> {
    let versions_result = sqlx::query!(
        r#"
        SELECT 
            p.id, p.name, p.version, p.description, p.file_size, p.published_at,
            u.id as owner_id, u.username as owner_username, u.email as owner_email, u.created_at as owner_created_at,
            t.id as team_id, t.name as team_name, t.description as team_description, t.owner_id as team_owner_id, t.created_at as team_created_at, t.updated_at as team_updated_at
        FROM packages p
        JOIN users u ON p.owner_id = u.id
        LEFT JOIN teams t ON p.team_id = t.id
        WHERE p.name = $1
        ORDER BY p.published_at DESC
        "#,
        name
    )
    .fetch_all(&mut **db)
    .await;

    match versions_result {
        Ok(rows) => {
            let versions: Vec<serde_json::Value> = rows
                .into_iter()
                .map(|row| {
                    let team = if let Some(team_id) = row.team_id {
                        Some(serde_json::json!({
                            "id": team_id,
                            "name": row.team_name,
                            "description": row.team_description,
                            "owner_id": row.team_owner_id,
                            "created_at": row.team_created_at,
                            "updated_at": row.team_updated_at
                        }))
                    } else {
                        None
                    };

                    serde_json::json!({
                        "id": row.id,
                        "name": row.name,
                        "version": row.version,
                        "description": row.description,
                        "file_size": row.file_size,
                        "published_at": row.published_at,
                        "owner": {
                            "id": row.owner_id,
                            "username": row.owner_username,
                            "email": row.owner_email,
                            "created_at": row.owner_created_at
                        },
                        "team": team
                    })
                })
                .collect();
            Ok(Json(versions))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/packages/<name>/<version>")]
pub async fn get_package(
    mut db: DbPool,
    name: &str,
    version: &str,
) -> Result<Json<serde_json::Value>, Status> {
    let package_result = sqlx::query!(
        r#"
        SELECT 
            p.id, p.name, p.version, p.description, p.download_url, p.file_size, p.checksum, p.published_at,
            u.id as owner_id, u.username as owner_username, u.email as owner_email, u.created_at as owner_created_at,
            t.id as team_id, t.name as team_name, t.description as team_description, t.owner_id as team_owner_id, t.created_at as team_created_at, t.updated_at as team_updated_at
        FROM packages p
        JOIN users u ON p.owner_id = u.id
        LEFT JOIN teams t ON p.team_id = t.id
        WHERE p.name = $1 AND p.version = $2
        "#,
        name,
        version
    )
    .fetch_optional(&mut **db)
    .await;

    match package_result {
        Ok(Some(row)) => {
            let team = if let Some(team_id) = row.team_id {
                Some(serde_json::json!({
                    "id": team_id,
                    "name": row.team_name,
                    "description": row.team_description,
                    "owner_id": row.team_owner_id,
                    "created_at": row.team_created_at,
                    "updated_at": row.team_updated_at
                }))
            } else {
                None
            };

            Ok(Json(serde_json::json!({
                "id": row.id,
                "name": row.name,
                "version": row.version,
                "description": row.description,
                "download_url": row.download_url,
                "file_size": row.file_size,
                "checksum": row.checksum,
                "published_at": row.published_at,
                "owner": {
                    "id": row.owner_id,
                    "username": row.owner_username,
                    "email": row.owner_email,
                    "created_at": row.owner_created_at
                },
                "team": team
            })))
        }
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/packages/download/<package_id>")]
pub async fn download_package(
    mut db: DbPool,
    package_id: &str,
) -> Result<rocket::fs::NamedFile, Status> {
    let package_id = Uuid::parse_str(package_id).map_err(|_| Status::BadRequest)?;
    // Get package info
    let package_result = sqlx::query!(
        "SELECT id, name, version FROM packages WHERE id = $1",
        package_id
    )
    .fetch_optional(&mut **db)
    .await;

    match package_result {
        Ok(Some(row)) => {
            let file_name = format!("{}-{}-{}.tar.gz", row.name, row.version, row.id);
            let file_path = format!("storage/packages/{}", file_name);

            match rocket::fs::NamedFile::open(&file_path).await {
                Ok(file) => Ok(file),
                Err(_) => Err(Status::NotFound),
            }
        }
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/packages/<name>/<version>")]
pub async fn delete_package(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
    name: &str,
    version: &str,
) -> Result<Status, Status> {
    // Check if user owns the package or is team admin/owner
    let package_result = sqlx::query!(
        r#"
        SELECT p.id, p.owner_id, p.team_id,
               CASE WHEN tm.role IS NOT NULL THEN tm.role ELSE NULL END as team_role
        FROM packages p
        LEFT JOIN team_members tm ON p.team_id = tm.team_id AND tm.user_id = $3
        WHERE p.name = $1 AND p.version = $2
        "#,
        name,
        version,
        auth_user.user_id
    )
    .fetch_optional(&mut **db)
    .await;

    let package_info = match package_result {
        Ok(Some(row)) => row,
        Ok(None) => return Err(Status::NotFound),
        Err(_) => return Err(Status::InternalServerError),
    };

    // Check permissions
    let can_delete = package_info.owner_id == auth_user.user_id
        || (package_info
            .team_role
            .as_ref()
            .map_or(false, |role| role == "owner" || role == "admin"));

    if !can_delete {
        return Err(Status::Forbidden);
    }

    // Delete package file
    let file_name = format!("{}-{}-{}.tar.gz", name, version, package_info.id);
    let file_path = format!("storage/packages/{}", file_name);
    let _ = std::fs::remove_file(&file_path); // Ignore errors

    // Delete package record
    let result = sqlx::query!(
        "DELETE FROM packages WHERE name = $1 AND version = $2",
        name,
        version
    )
    .execute(&mut **db)
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => Ok(Status::NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
