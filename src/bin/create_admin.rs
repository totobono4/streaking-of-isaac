// Usage: cargo run --bin create_admin -- <username> <password>
// Create (or update) admin user directly in base.
// Only way to add admins, by design, no online inscription.
// public, we give access ourselves to trusted members.

use streaking_of_isaac::{auth, db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: create_admin <username> <password>");
        std::process::exit(1);
    }
    let username = &args[1];
    let password = &args[2];

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db".to_string());
    let pool = db::init_pool(&database_url).await?;

    let hash = auth::hash_password(password)?;

    sqlx::query(
        "INSERT INTO users (username, password_hash, is_admin) VALUES (?, ?, 1)
         ON CONFLICT(username) DO UPDATE SET password_hash = excluded.password_hash, is_admin = 1",
    )
    .bind(username)
    .bind(hash)
    .execute(&pool)
    .await?;

    println!("Admin '{username}' created/updated.");
    Ok(())
}
