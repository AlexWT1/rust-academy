//! Авторизация: хеширование пароля, сессионный cookie, извлечение текущего юзера.

use axum::http::header;

use crate::db::Db;

/// Cookie сессионного токена.
pub const COOKIE: &str = "ra_session";
/// Сессия живёт 30 дней.
const SESSION_TTL_SECS: i64 = 30 * 24 * 3600;

/// Текущий пользователь, вставляется middleware в request extensions.
#[derive(Clone)]
pub struct CurrentUser {
    pub id: i64,
    pub username: String,
}

/// sha256(salt || password) в hex. Простая, но приличная защита для личного курса.
pub fn hash_password(password: &str) -> (String, String) {
    let salt = random_token()[..16].to_string();
    let hash = hash_password_with_salt(&salt, password);
    (hash, salt)
}

pub fn hash_password_with_salt(salt: &str, password: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());
    hex::encode(hasher.finalize())
}

/// Случайный hex-токен (32 байта энтропии) для cookie и соли.
pub fn random_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Достать токен сессии из заголовка Cookie.
pub fn session_token(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookies = headers.get(header::COOKIE)?.to_str().ok()?;
    cookies.split(';').find_map(|pair| {
        let pair = pair.trim();
        pair.strip_prefix(COOKIE)
            .and_then(|rest| rest.strip_prefix('='))
            .map(|value| value.to_string())
    })
}

/// Текущий пользователь по заголовкам запроса.
pub fn current_user(db: &Db, headers: &axum::http::HeaderMap) -> Option<CurrentUser> {
    let token = session_token(headers)?;
    let (id, username) = crate::db::user_by_session(db, &token)?;
    Some(CurrentUser { id, username })
}

/// Заголовок Set-Cookie для новой сессии.
pub fn set_session_cookie(token: &str) -> String {
    format!(
        "{COOKIE}={token}; HttpOnly; Path=/; SameSite=Lax; Max-Age={SESSION_TTL_SECS}"
    )
}

/// Заголовок Set-Cookie, сбрасывающий сессию.
pub fn clear_session_cookie() -> String {
    format!("{COOKIE}=; HttpOnly; Path=/; SameSite=Lax; Max-Age=0")
}
