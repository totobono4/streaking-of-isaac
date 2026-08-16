use crate::state::AppState;
use crate::auth;
use crate::db;

use askama_axum::IntoResponse;
use axum::extract::State;
use axum::extract::Path;
use axum::response::Redirect;
use axum::Form;
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct NewEntryForm {
    player_name: String,
    player_link: Option<String>,
    score: i64,
    stat_text: String,
    stat_link: Option<String>,
    note: String,
}

pub async fn upsert_entry_to_leaderboard(
    State(state): State<AppState>,
    session: Session,
    Path(slug): Path<String>,
    Form(form): Form<NewEntryForm>,
) -> impl IntoResponse {
    if !auth::is_admin(&session).await {
        return Redirect::to("/admin/login");
    }
    let user_id: i64 = session
        .get(auth::SESSION_USER_ID)
        .await
        .ok()
        .flatten()
        .unwrap_or(0);

    if let Ok(Some(lb)) = db::get_leaderboard_by_slug(&state.pool, &slug).await {
        let player_link = form.player_link.filter(|u| !u.trim().is_empty());
        let stat_text = if form.stat_text.trim().is_empty() { "???".to_string() } else { form.stat_text };
        let stat_link = form.stat_link.filter(|u| !u.trim().is_empty());

        db::add_and_update_entry(
            &state.pool,
            lb.id,
            &form.player_name,
            player_link.as_deref(),
            form.score,
            &stat_text,
            stat_link.as_deref(),
            &form.note,
            user_id,
        )
        .await
        .ok();
    }

    Redirect::to(&format!("/leaderboard/{}", slug))
}

#[derive(Deserialize)]
pub struct RemoveEntryForm {
    id: i64,
}

pub async fn remove_entry(
    State(state): State<AppState>,
    session: Session,
    Path(slug): Path<String>,
    form: Form<RemoveEntryForm>,
) -> impl IntoResponse {
    if !auth::is_admin(&session).await {
        return Redirect::to("/admin/login");
    }

    db::remove_entry_by_id(&state.pool, form.id).await.ok();

    Redirect::to(&format!("/leaderboard/{}", slug))
}
