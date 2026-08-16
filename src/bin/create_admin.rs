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

    db::create_user(&pool, &username, &hash, true).await.ok();

    println!("Admin '{username}' created/updated.");
    Ok(())
}
