#[macro_use]
extern crate rocket;

mod auth;
mod handlers;
mod models;
mod validation;

use auth::KnotDb;
use handlers::{packages, teams, users};
use rocket::serde::json::Json;
use rocket_db_pools::Database;

#[get("/")]
fn index() -> &'static str {
    "Knot Space - Online Package Repository Server"
}

#[get("/health")]
fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "knot-space",
        "version": "0.1.0"
    }))
}

async fn run_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Running database migrations...");
    
    let pool = sqlx::PgPool::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    println!("✅ Migrations completed successfully");
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    // Run migrations before starting the server
    if let Err(e) = run_migrations(&database_url).await {
        eprintln!("❌ Failed to run migrations: {}", e);
        std::process::exit(1);
    }

    // Build and launch the rocket
    let _rocket = rocket::build()
        .attach(KnotDb::init())
        .mount("/", routes![index, health])
        .mount(
            "/api/auth",
            routes![
                users::register,
                users::login,
                users::get_profile,
                users::delete_account,
                users::get_user_by_username
            ],
        )
        .mount(
            "/api",
            routes![
                teams::create_team,
                teams::list_teams,
                teams::get_team,
                teams::get_team_members,
                teams::add_team_member,
                teams::remove_team_member,
                teams::delete_team
            ],
        )
        .mount(
            "/api",
            routes![
                packages::publish_package,
                packages::list_packages,
                packages::get_package_versions,
                packages::get_package,
                packages::download_package,
                packages::delete_package
            ],
        )
        .launch()
        .await?;

    Ok(())
}
