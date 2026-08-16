use crate::db::Leaderboard;
use crate::state::AppState;
use crate::auth;
use crate::db;

use axum::response::IntoResponse;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Redirect;
use axum::Form;
use serde::Deserialize;
use tower_sessions::Session;

#[derive(askama::Template, askama_web::WebTemplate)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
    is_admin: bool,
}

pub async fn login_form(
    session: Session
) -> impl IntoResponse {
    let is_admin = auth::is_admin(&session).await;
    LoginTemplate {
        error: None,
        is_admin,
    }
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_submit(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    match db::find_user_by_username(&state.pool, &form.username).await {
        Ok(Some(user)) if auth::verify_password(&form.password, &user.password_hash) => {
            session
                .insert(auth::SESSION_USER_ID, user.id)
                .await
                .ok();
            session
                .insert(auth::SESSION_IS_ADMIN, user.is_admin)
                .await
                .ok();
            Redirect::to("/admin").into_response()
        }
        _ => LoginTemplate {
            error: Some("Identifiants invalides".to_string()),
            is_admin: false,
        }
        .into_response(),
    }
}

pub async fn logout(session: Session) -> impl IntoResponse {
    session.flush().await.ok();
    Redirect::to("/")
}

#[derive(Deserialize)]
pub struct CreateAdminForm {
    username: String,
    password: String,
}

pub async fn create_admin(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<CreateAdminForm>,
) -> impl IntoResponse {
    if !auth::is_admin(&session).await {
        return Redirect::to("/admin").into_response();
    }
    if let Ok(hash) = auth::hash_password(&form.password) {
        db::create_user(&state.pool, &form.username, &hash, true).await.ok();
    }
    Redirect::to("/admin").into_response()
}

#[derive(Deserialize)]
pub struct RemoveAdminForm {
    username: String,
}

pub async fn remove_admin(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<RemoveAdminForm>,
) -> impl IntoResponse {
    if !auth::is_admin(&session).await {
        return Redirect::to("/admin").into_response();
    }
    db::remove_user(&state.pool, &form.username).await.ok();
    Redirect::to("/admin").into_response()
}

#[derive(Deserialize)]
pub struct PrefillLeaderboardParams {
    slug: Option<String>,
}

#[derive(askama::Template, askama_web::WebTemplate)]
#[template(path = "admin.html")]
struct AdminTemplate {
    leaderboards: Vec<db::Leaderboard>,
    is_admin: bool,
    admins: Vec<db::User>,

    prefill_leaderboard: Option<Leaderboard>,
}

pub async fn admin_panel(
    State(state): State<AppState>,
    session: Session,
    Query(params): Query<PrefillLeaderboardParams>,
) -> impl IntoResponse {
    if !auth::is_admin(&session).await {
        return Redirect::to("/admin/login").into_response();
    }
    let leaderboards = db::list_leaderboards(&state.pool).await.unwrap_or_default();
    let admins = db::list_users(&state.pool).await.unwrap_or_default();

    let prefill_leaderboard = if let Some(slug) = params.slug {
        db::get_leaderboard_by_slug(&state.pool, &slug).await.unwrap_or(None)
    } else { None };
    AdminTemplate {
        leaderboards,
        is_admin: true,
        admins,

        prefill_leaderboard,
    }
    .into_response()
}
