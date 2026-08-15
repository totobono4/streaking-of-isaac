use crate::state::AppState;
use crate::auth;
use crate::db;
use crate::db::Entry;

use askama_axum::IntoResponse;
use axum::extract::Query;
use axum::extract::State;
use axum::extract::Path;
use axum::response::Redirect;
use axum::Form;
use serde::Deserialize;
use tower_sessions::Session;

#[derive(askama::Template)]
#[template(path = "leaderboards.html")]
struct LeaderBoardsTemplate {
    leaderboards: Vec<db::Leaderboard>,
    is_admin: bool,
}

pub async fn leaderboards(
    State(state): State<AppState>,
    session: Session
) -> impl IntoResponse {
    let leaderboards = db::list_leaderboards(&state.pool).await.unwrap_or_default();
    let is_admin = auth::is_admin(&session).await;
    LeaderBoardsTemplate {
        leaderboards,
        is_admin,
    }
}

#[derive(Deserialize)]
pub struct PrefillParams {
    edit_id: Option<i64>,
}

#[derive(askama::Template)]
#[template(path = "leaderboard.html")]
struct LeaderboardTemplate {
    leaderboard: db::Leaderboard,
    entries: Vec<db::Entry>,
    is_admin: bool,

    prefill_entry: Option<Entry>,
}

pub async fn view_leaderboard(
    State(state): State<AppState>,
    session: Session,
    Path(slug): Path<String>,
    Query(params): Query<PrefillParams>,
) -> impl IntoResponse {
    let is_admin = auth::is_admin(&session).await;
    match db::get_leaderboard_by_slug(&state.pool, &slug).await {
        Ok(Some(lb)) => {
            let entries = db::list_entries(&state.pool, lb.id, lb.lower_is_better)
                .await
                .unwrap_or_default();
            let prefill_entry = if let Some(id) = params.edit_id {
                db::get_entry_by_id(&state.pool, id).await.unwrap_or(None)
            } else { None };
            LeaderboardTemplate {
                leaderboard: lb,
                entries,
                is_admin,

                prefill_entry,
            }
            .into_response()
        }
        _ => (axum::http::StatusCode::NOT_FOUND, "Leaderboard introuvable").into_response(),
    }
}

#[derive(Deserialize)]
pub struct NewLeaderboardForm {
    slug: String,
    title: String,
    description: Option<String>,
    unit: String,
    stat: String,
    lower_is_better: Option<String>,
}

pub async fn create_leaderboard(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<NewLeaderboardForm>,
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

    let lower_is_better = form.lower_is_better.is_some();
    let description = form.description.filter(|d| !d.trim().is_empty());
    let unit = if form.unit.trim().is_empty() { "Best Streak".to_string() } else { form.unit };
    let stat = if form.stat.trim().is_empty() { "Status".to_string() } else { form.stat };

    db::create_leaderboard(
        &state.pool,
        &form.slug,
        &form.title,
        description.as_deref(),
        &unit,
        &stat,
        lower_is_better,
        user_id,
    )
    .await
    .ok();

    Redirect::to("/admin")
}
