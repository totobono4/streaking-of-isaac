use crate::auth;

use askama_axum::IntoResponse;
use tower_sessions::Session;

#[derive(askama::Template)]
#[template(path = "rules.html")]
struct RulesTemplate {
    is_admin: bool,
}

pub async fn rules(
    session: Session
) -> impl IntoResponse {
    let is_admin = auth::is_admin(&session).await;
    RulesTemplate {
        is_admin,
    }
}
