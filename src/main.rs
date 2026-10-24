mod auth;
mod content;
mod db;
mod render;

use axum::extract::{Path, Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Extension, Form, Json, Router};
use serde::Deserialize;

use crate::auth::CurrentUser;
use crate::db::Db;

static APP_CSS: &str = include_str!("../static/app.css");
static APP_JS: &str = include_str!("../static/app.js");

#[derive(Clone)]
struct AppState {
    catalog: render::Catalog,
    db: Db,
}

#[tokio::main]
async fn main() {
    let db_path = std::env::var("RA_DB").unwrap_or_else(|_| "rust-academy.db".to_string());
    let db = Db::open(&db_path);

    // Стартовая учётка: admin/admin, пароль можно переопределить через RA_ADMIN_PASSWORD.
    let admin_password =
        std::env::var("RA_ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    db.seed_admin("admin", &admin_password);

    let state = AppState {
        catalog: render::Catalog::build(content::course()),
        db,
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/lesson/{id}", get(lesson))
        .route("/login", get(login_page).post(login_submit))
        .route("/logout", post(logout))
        .route("/api/progress", get(progress_get).post(progress_post))
        .route("/static/app.css", get(css))
        .route("/static/app.js", get(js))
        .fallback(not_found)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8011);
    // HOST=0.0.0.0 — слушать все интерфейсы (нужно на сервере);
    // по умолчанию — только локальный хост.
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let listener = tokio::net::TcpListener::bind((host.as_str(), port))
        .await
        .expect("не удалось занять порт");
    println!();
    println!("  🦀 Rust Academy запущена: http://{host}:{port}");
    println!("  Ctrl+C — остановить сервер");
    println!();
    axum::serve(listener, app).await.expect("ошибка сервера");
}

/// Все маршруты, кроме /login и статики, требуют валидную сессию.
async fn auth_middleware(State(state): State<AppState>, mut req: Request, next: Next) -> Response {
    let path = req.uri().path();
    if path == "/login" || path.starts_with("/static/") {
        return next.run(req).await;
    }
    match auth::current_user(&state.db, req.headers()) {
        Some(user) => {
            req.extensions_mut().insert(user);
            next.run(req).await
        }
        None => Redirect::to("/login").into_response(),
    }
}

async fn home(State(state): State<AppState>, Extension(user): Extension<CurrentUser>) -> Html<String> {
    Html(render::render_home(&state.catalog, Some(&user.username)))
}

async fn lesson(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
) -> Response {
    match state.catalog.find(&id, Some(&user.username)) {
        Some(page) => Html(page).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Html(state.catalog.not_found_page(Some(&user.username))),
        )
            .into_response(),
    }
}

async fn css() -> Response {
    static_file(APP_CSS, "text/css; charset=utf-8")
}

async fn js() -> Response {
    static_file(APP_JS, "application/javascript; charset=utf-8")
}

fn static_file(body: &'static str, content_type: &'static str) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, content_type), (header::CACHE_CONTROL, "max-age=3600")],
        body,
    )
        .into_response()
}

// ---------- Вход и выход ----------

async fn login_page(State(state): State<AppState>, headers: axum::http::HeaderMap) -> Response {
    // Уже вошёл — на главную.
    if auth::current_user(&state.db, &headers).is_some() {
        return Redirect::to("/").into_response();
    }
    Html(render::render_login(None)).into_response()
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login_submit(State(state): State<AppState>, Form(form): Form<LoginForm>) -> Response {
    match db::verify_login(&state.db, &form.username, &form.password) {
        Some(user_id) => {
            let token = db::create_session(&state.db, user_id);
            let mut resp = Redirect::to("/").into_response();
            resp.headers_mut().insert(
                header::SET_COOKIE,
                auth::set_session_cookie(&token).parse().expect("valid cookie header"),
            );
            resp
        }
        None => Html(render::render_login(Some("Неверный логин или пароль"))).into_response(),
    }
}

async fn logout(State(state): State<AppState>, headers: axum::http::HeaderMap) -> Response {
    if let Some(token) = auth::session_token(&headers) {
        db::delete_session(&state.db, &token);
    }
    let mut resp = Redirect::to("/login").into_response();
    resp.headers_mut().insert(
        header::SET_COOKIE,
        auth::clear_session_cookie().parse().expect("valid cookie header"),
    );
    resp
}

// ---------- Синхронизация прогресса ----------

async fn progress_get(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
) -> Json<Vec<String>> {
    Json(db::progress_of(&state.db, user.id))
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ProgressReq {
    Single { lesson_id: String, done: bool },
    Batch { items: Vec<ProgressItem> },
}

#[derive(Deserialize)]
struct ProgressItem {
    lesson_id: String,
    done: bool,
}

async fn progress_post(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
    Json(req): Json<ProgressReq>,
) -> StatusCode {
    match req {
        ProgressReq::Single { lesson_id, done } => {
            db::set_progress(&state.db, user.id, &lesson_id, done);
        }
        ProgressReq::Batch { items } => {
            for item in items {
                db::set_progress(&state.db, user.id, &item.lesson_id, item.done);
            }
        }
    }
    StatusCode::OK
}

async fn not_found(State(state): State<AppState>, headers: axum::http::HeaderMap) -> Response {
    let username = auth::current_user(&state.db, &headers).map(|u| u.username);
    (StatusCode::NOT_FOUND, Html(state.catalog.not_found_page(username.as_deref()))).into_response()
}
