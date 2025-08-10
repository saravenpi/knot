use crate::auth::{AuthenticatedUser, DbPool};
use crate::models::{AddTeamMemberRequest, CreateTeamRequest, Team};
use chrono::Utc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post};
use uuid::Uuid;
use validator::Validate;

#[post("/teams", data = "<request>")]
pub async fn create_team(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
    request: Json<CreateTeamRequest>,
) -> Result<Json<Team>, Status> {
    // Validate input
    if let Err(_) = request.validate() {
        return Err(Status::BadRequest);
    }

    // Check if team name already exists
    let existing_team = sqlx::query!("SELECT id FROM teams WHERE name = $1", request.name)
        .fetch_optional(&mut **db)
        .await;

    match existing_team {
        Ok(Some(_)) => return Err(Status::Conflict),
        Ok(None) => {}
        Err(_) => return Err(Status::InternalServerError),
    }

    let team_id = Uuid::new_v4();
    let now = Utc::now();

    // Create team
    let result = sqlx::query_as!(
        Team,
        r#"
        INSERT INTO teams (id, name, description, owner_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, description, owner_id, created_at, updated_at
        "#,
        team_id,
        request.name,
        request.description,
        auth_user.user_id,
        now,
        now
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(team) => {
            // Add owner as team member with owner role
            let member_result = sqlx::query!(
                r#"
                INSERT INTO team_members (id, team_id, user_id, role, joined_at)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                Uuid::new_v4(),
                team_id,
                auth_user.user_id,
                "owner",
                now
            )
            .execute(&mut **db)
            .await;

            match member_result {
                Ok(_) => Ok(Json(team)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/teams")]
pub async fn list_teams(mut db: DbPool) -> Result<Json<Vec<Team>>, Status> {
    let teams_result = sqlx::query_as!(
        Team,
        "SELECT id, name, description, owner_id, created_at, updated_at FROM teams ORDER BY name"
    )
    .fetch_all(&mut **db)
    .await;

    match teams_result {
        Ok(teams) => Ok(Json(teams)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/teams/<team_name>")]
pub async fn get_team(mut db: DbPool, team_name: &str) -> Result<Json<Team>, Status> {
    let team_result = sqlx::query_as!(
        Team,
        "SELECT id, name, description, owner_id, created_at, updated_at FROM teams WHERE name = $1",
        team_name
    )
    .fetch_optional(&mut **db)
    .await;

    match team_result {
        Ok(Some(team)) => Ok(Json(team)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/teams/<team_name>/members")]
pub async fn get_team_members(
    mut db: DbPool,
    team_name: &str,
) -> Result<Json<Vec<serde_json::Value>>, Status> {
    let members_result = sqlx::query!(
        r#"
        SELECT 
            tm.role,
            tm.joined_at,
            u.id as user_id,
            u.username,
            u.email,
            u.created_at as user_created_at
        FROM team_members tm
        JOIN users u ON tm.user_id = u.id
        JOIN teams t ON tm.team_id = t.id
        WHERE t.name = $1
        ORDER BY tm.joined_at
        "#,
        team_name
    )
    .fetch_all(&mut **db)
    .await;

    match members_result {
        Ok(rows) => {
            let members: Vec<serde_json::Value> = rows
                .into_iter()
                .map(|row| {
                    serde_json::json!({
                        "role": row.role,
                        "joined_at": row.joined_at,
                        "user": {
                            "id": row.user_id,
                            "username": row.username,
                            "email": row.email,
                            "created_at": row.user_created_at
                        }
                    })
                })
                .collect();
            Ok(Json(members))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/teams/<team_name>/members", data = "<request>")]
pub async fn add_team_member(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
    team_name: &str,
    request: Json<AddTeamMemberRequest>,
) -> Result<Status, Status> {
    // Check if user is team owner or admin
    let permission_check = sqlx::query!(
        r#"
        SELECT tm.role
        FROM team_members tm
        JOIN teams t ON tm.team_id = t.id
        WHERE t.name = $1 AND tm.user_id = $2
        "#,
        team_name,
        auth_user.user_id
    )
    .fetch_optional(&mut **db)
    .await;

    let user_role = match permission_check {
        Ok(Some(row)) => row.role,
        Ok(None) => return Err(Status::Forbidden),
        Err(_) => return Err(Status::InternalServerError),
    };

    if user_role != "owner" && user_role != "admin" {
        return Err(Status::Forbidden);
    }

    // Find the team and user to add
    let team_result = sqlx::query!("SELECT id FROM teams WHERE name = $1", team_name)
        .fetch_optional(&mut **db)
        .await;

    let team_id = match team_result {
        Ok(Some(row)) => row.id,
        Ok(None) => return Err(Status::NotFound),
        Err(_) => return Err(Status::InternalServerError),
    };

    let user_result = sqlx::query!("SELECT id FROM users WHERE username = $1", request.username)
        .fetch_optional(&mut **db)
        .await;

    let user_id = match user_result {
        Ok(Some(row)) => row.id,
        Ok(None) => return Err(Status::NotFound),
        Err(_) => return Err(Status::InternalServerError),
    };

    // Check if user is already a member
    let existing_member = sqlx::query!(
        "SELECT id FROM team_members WHERE team_id = $1 AND user_id = $2",
        team_id,
        user_id
    )
    .fetch_optional(&mut **db)
    .await;

    match existing_member {
        Ok(Some(_)) => return Err(Status::Conflict),
        Ok(None) => {}
        Err(_) => return Err(Status::InternalServerError),
    }

    // Add user to team
    let result = sqlx::query!(
        r#"
        INSERT INTO team_members (id, team_id, user_id, role, joined_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        team_id,
        user_id,
        request.role,
        Utc::now()
    )
    .execute(&mut **db)
    .await;

    match result {
        Ok(_) => Ok(Status::Created),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/teams/<team_name>/members/<username>")]
pub async fn remove_team_member(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
    team_name: &str,
    username: &str,
) -> Result<Status, Status> {
    // Check if user is team owner or admin
    let permission_check = sqlx::query!(
        r#"
        SELECT tm.role
        FROM team_members tm
        JOIN teams t ON tm.team_id = t.id
        WHERE t.name = $1 AND tm.user_id = $2
        "#,
        team_name,
        auth_user.user_id
    )
    .fetch_optional(&mut **db)
    .await;

    let user_role = match permission_check {
        Ok(Some(row)) => row.role,
        Ok(None) => return Err(Status::Forbidden),
        Err(_) => return Err(Status::InternalServerError),
    };

    if user_role != "owner" && user_role != "admin" {
        return Err(Status::Forbidden);
    }

    // Remove user from team
    let result = sqlx::query!(
        r#"
        DELETE FROM team_members
        WHERE team_id = (SELECT id FROM teams WHERE name = $1)
        AND user_id = (SELECT id FROM users WHERE username = $2)
        AND role != 'owner'
        "#,
        team_name,
        username
    )
    .execute(&mut **db)
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => Ok(Status::NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/teams/<team_name>")]
pub async fn delete_team(
    mut db: DbPool,
    auth_user: AuthenticatedUser,
    team_name: &str,
) -> Result<Status, Status> {
    // Only team owner can delete team
    let result = sqlx::query!(
        "DELETE FROM teams WHERE name = $1 AND owner_id = $2",
        team_name,
        auth_user.user_id
    )
    .execute(&mut **db)
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => Ok(Status::NoContent),
        Ok(_) => Err(Status::Forbidden),
        Err(_) => Err(Status::InternalServerError),
    }
}
