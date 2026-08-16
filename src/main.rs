use streaking_of_isaac::state::AppState;
use streaking_of_isaac::db;
use streaking_of_isaac::routes::*;

use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Router;
use tower_http::services::ServeDir;
use tower_sessions::Expiry;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::SqliteStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db".to_string());
    let pool = db::init_pool(&database_url).await?;

    let session_store = SqliteStore::new(pool.clone());
    session_store.migrate().await?;
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(30)));

    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(index))
        .route("/rules", get(rules))
        .route("/leaderboards", get(leaderboards))
        .route("/leaderboard/{slug}", get(view_leaderboard))
        .route("/admin/login", get(login_form).post(login_submit))
        .route("/admin/logout", get(logout))
        .route("/admin/create", post(create_admin))
        .route("/admin/remove", post(remove_admin))
        .route("/admin", get(admin_panel))
        .route("/admin/leaderboard/create", post(create_leaderboard))
        .route("/admin/leaderboard/update", post(update_leaderboard))
        .route("/admin/leaderboard/remove", post(remove_leaderboard))
        .route("/admin/leaderboard/{slug}/entries/upsert", post(upsert_entry_to_leaderboard))
        .route("/admin/leaderboard/{slug}/entries/remove", post(remove_entry))
        .nest_service("/static", ServeDir::new("static"))
        .layer(session_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Serveur lancé sur http://0.0.0.0:8080");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    Redirect::to("/rules")
}
