//! SQLite: пользователи, сессии, прогресс. Файл БД — рядом с бинарником.

use std::sync::Mutex;

use rusqlite::{params, Connection};

use crate::auth;

/// Общая обёртка над подключением (однопоточный доступ под мьютексом).
#[derive(Clone)]
pub struct Db(pub std::sync::Arc<Mutex<Connection>>);

impl Db {
    pub fn open(path: &str) -> Db {
        let conn = Connection::open(path).expect("не удалось открыть SQLite-базу");
        conn.execute_batch(SCHEMA).expect("не удалось создать схему");
        Db(std::sync::Arc::new(Mutex::new(conn)))
    }

    pub fn seed_admin(&self, username: &str, password: &str) {
        let conn = self.0.lock().unwrap();
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?1)",
                params![username],
                |r| r.get(0),
            )
            .unwrap_or(false);
        if !exists {
            let (hash, salt) = auth::hash_password(password);
            conn.execute(
                "INSERT INTO users (username, pass_hash, salt) VALUES (?1, ?2, ?3)",
                params![username, hash, salt],
            )
            .expect("не удалось создать учётку admin");
            println!("  создана учётка: {username}");
        }
    }
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    username  TEXT UNIQUE NOT NULL,
    pass_hash TEXT NOT NULL,
    salt      TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    token      TEXT PRIMARY KEY,
    user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS progress (
    user_id  INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lesson_id TEXT NOT NULL,
    done_at  INTEGER NOT NULL,
    PRIMARY KEY (user_id, lesson_id)
);
"#;

/// Проверка логина/пароля. Возвращает user_id при успехе.
pub fn verify_login(db: &Db, username: &str, password: &str) -> Option<i64> {
    let conn = db.0.lock().unwrap();
    conn.query_row(
        "SELECT id, pass_hash, salt FROM users WHERE username = ?1",
        params![username],
        |row| {
            let id: i64 = row.get(0)?;
            let hash: String = row.get(1)?;
            let salt: String = row.get(2)?;
            Ok((id, hash, salt))
        },
    )
    .ok()
    .and_then(|(id, hash, salt)| {
        if auth::hash_password_with_salt(&salt, password) == hash {
            Some(id)
        } else {
            None
        }
    })
}

/// Создать сессию, вернуть токен для cookie.
pub fn create_session(db: &Db, user_id: i64) -> String {
    let token = auth::random_token();
    let now = now_secs();
    db.0
        .lock()
        .unwrap()
        .execute(
            "INSERT INTO sessions (token, user_id, created_at) VALUES (?1, ?2, ?3)",
            params![token, user_id, now],
        )
        .expect("insert session");
    token
}

/// Имя пользователя по токену сессии (или None, если сессии нет/устарела).
pub fn user_by_session(db: &Db, token: &str) -> Option<(i64, String)> {
    let conn = db.0.lock().unwrap();
    conn.query_row(
        "SELECT u.id, u.username FROM sessions s JOIN users u ON u.id = s.user_id WHERE s.token = ?1",
        params![token],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
            ))
        },
    )
    .ok()
}

pub fn delete_session(db: &Db, token: &str) {
    let _ = db.0.lock().unwrap().execute(
        "DELETE FROM sessions WHERE token = ?1",
        params![token],
    );
}

/// Пройденные уроки пользователя.
pub fn progress_of(db: &Db, user_id: i64) -> Vec<String> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT lesson_id FROM progress WHERE user_id = ?1 ORDER BY done_at")
        .expect("prepare progress");
    let rows = stmt
        .query_map(params![user_id], |r| r.get::<_, String>(0))
        .expect("query progress");
    rows.filter_map(|r| r.ok()).collect()
}

/// Отметить/снять прохождение урока.
pub fn set_progress(db: &Db, user_id: i64, lesson_id: &str, done: bool) {
    let conn = db.0.lock().unwrap();
    if done {
        let _ = conn.execute(
            "INSERT OR IGNORE INTO progress (user_id, lesson_id, done_at) VALUES (?1, ?2, ?3)",
            params![user_id, lesson_id, now_secs()],
        );
    } else {
        let _ = conn.execute(
            "DELETE FROM progress WHERE user_id = ?1 AND lesson_id = ?2",
            params![user_id, lesson_id],
        );
    }
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
