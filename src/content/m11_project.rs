//! Модуль 11: Проект — CLI-планировщик задач и REST API.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🏗",
        title: "Проект: менеджер задач",
        blurb: "Финальный проект курса: CLI-приложение с JSON-хранением, а затем REST API на Axum.",
        lessons: vec![cli_design(), cli_impl(), api_project()],
    }
}

fn cli_design() -> Lesson {
    Lesson {
        id: "project-cli",
        title: "Проект 1: CLI-планировщик — дизайн",
        blocks: vec![
            Block::P("Пора собрать всё изученное в работающее приложение. Проект: консольный менеджер задач (todo) с сохранением в JSON. Он задействует структуры, enum, Option/Result, итераторы, трейты и модули — весь курс на практике."),
            Block::Heading("Требования"),
            Block::List(&[
                "add \"текст\" [--prio N] — добавить задачу с приоритетом.",
                "list [--all] — показать открытые (или все) задачи.",
                "done N — закрыть задачу по номеру.",
                "rm N — удалить задачу.",
                "Данные живут в tasks.json рядом с бинарником; переносимы между запусками.",
            ]),
            Block::Heading("Моделирование данных"),
            Block::Code {
                caption: "src/models.rs — домен приложения",
                code: r##"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub prio: u8,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, text: impl Into<String>, prio: u8) -> Self {
        Task { id, text: text.into(), prio, done: false }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    pub next_id: u32,
    pub tasks: Vec<Task>,
}

impl Store {
    pub fn add(&mut self, text: String, prio: u8) -> &Task {
        let id = self.next_id;
        self.next_id += 1;
        let task = Task::new(id, text, prio);
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    pub fn mark_done(&mut self, id: u32) -> Result<&Task, String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| format!("задача {id} не найдена"))?;
        task.done = true;
        Ok(task)
    }
}"##,
                runnable: false,
            },
            Block::Heading("Зависимости проекта"),
            Block::Code {
                caption: "Cargo.toml",
                code: r##"[package]
name = "todo"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1""##,
                runnable: false,
            },
            Block::Note("serde — стандарт сериализации: derive(Serialize, Deserialize) превращает ваши структуры в JSON и обратно. serde_json — парсер/форматтер. Две самые популярные зависимости экосистемы."),
            Block::Heading("Архитектура"),
            Block::Steps(&[
                "models.rs — типы Task и Store с логикой (без ввода-вывода!).",
                "storage.rs — загрузка/сохранение JSON (Result-обработка).",
                "cli.rs — разбор аргументов, вывод на экран.",
                "main.rs — склейка: аргументы -> действие -> хранилище.",
                "Такое разделение позволит позже заменить CLI на REST API без переписывания домена.",
            ]),
        ],
        quiz: vec![
            Question {
                text: "Почему логика Store отделена от работы с файлами?",
                options: &[
                    "Так меньше строк кода",
                    "Домен тестируется без файловой системы и переиспользуется (CLI или API — неважно)",
                    "Иначе не компилируется",
                ],
                answer: 1,
                explain: "Разделение домена и инфраструктуры — ключ к тестируемости и переиспользованию. Именно так мы подключим API в проекте 2.",
            },
            Question {
                text: "Что делает derive(Serialize, Deserialize) из serde?",
                options: &[
                    "Генерирует JSON-код для структуры на этапе компиляции",
                    "Шифрует данные",
                    "Ничего, это комментарий",
                ],
                answer: 0,
                explain: "Процедурный макрос serde генерирует реализации трейтов Serialize/Deserialize для вашего типа.",
            },
        ],
        exercise: Some(Exercise {
            task: "Создайте проект: cargo new todo. Добавьте serde-зависимости (cargo add serde --features derive; cargo add serde_json). Напишите models.rs из урока и убедитесь, что cargo check проходит.",
            starter: r##"# Терминал:
# cargo new todo && cd todo
# cargo add serde --features derive
# cargo add serde_json
# touch src/models.rs
# // затем перенесите код models.rs и добавьте в main.rs:
# mod models;
# fn main() {
#     let mut store = todo::models::Store::default();
# }"##,
            solution: r##"// src/main.rs
mod models;

use crate::models::Store;

fn main() {
    let mut store = Store::default();
    let task = store.add(String::from("пройти курс Rust"), 1);
    println!("{task:?}");

    store.mark_done(task.id).unwrap();
    println!("{:?}", store.tasks);
}"##,
        }),
    }
}

fn cli_impl() -> Lesson {
    Lesson {
        id: "project-cli2",
        title: "Проект 1: реализация CLI",
        blocks: vec![
            Block::Heading("Хранение: storage.rs"),
            Block::Code {
                caption: "Загрузка и сохранение JSON",
                code: r##"use std::fs;
use std::path::Path;

use serde_json::{from_str, to_string_pretty};

use crate::models::Store;

const FILE: &str = "tasks.json";

pub fn load() -> Store {
    if !Path::new(FILE).exists() {
        return Store::default();
    }
    // Result обрабатываем честно: файл может быть битым
    match fs::read_to_string(FILE) {
        Ok(data) => from_str(&data).unwrap_or_default(),
        Err(_) => Store::default(),
    }
}

pub fn save(store: &Store) -> std::io::Result<()> {
    fs::write(FILE, to_string_pretty(store)?)?;
    Ok(())
}"##,
                runnable: false,
            },
            Block::Heading("Интерфейс: cli.rs"),
            Block::Code {
                caption: "Разбор аргументов и вывод",
                code: r##"use crate::models::Store;

pub fn run(store: &mut Store, args: &[String]) {
    match args.first().map(String::as_str) {
        Some("add") => {
            let text = args.get(1).cloned().unwrap_or_else(|| "безымянная".into());
            let prio: u8 = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
            let t = store.add(text, prio);
            println!("✔ добавлена #{}: {}", t.id, t.text);
        }
        Some("list") => {
            for t in &store.tasks {
                let mark = if t.done { "✔" } else { " " };
                println!("#{} [{mark}] {} (p{})", t.id, t.text, t.prio);
            }
            let open = store.tasks.iter().filter(|t| !t.done).count();
            println!("открытых: {open}");
        }
        Some("done") => {
            if let Some(id) = args.get(1).and_then(|s| s.parse().ok()) {
                match store.mark_done(id) {
                    Ok(t) => println!("✔ закрыта: {}", t.text),
                    Err(e) => eprintln!("ошибка: {e}"),
                }
            }
        }
        _ => println!("команды: add \"текст\", list, done N, rm N"),
    }
}"##,
                runnable: false,
            },
            Block::Heading("main.rs: склейка"),
            Block::Code {
                caption: "Точка входа",
                code: r##"mod cli;
mod models;
mod storage;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut store = storage::load();

    cli::run(&mut store, &args);

    if let Err(e) = storage::save(&store) {
        eprintln!("не удалось сохранить: {e}");
        std::process::exit(1);
    }
}"##,
                runnable: false,
            },
            Block::Heading("Работа с приложением"),
            Block::Code {
                caption: "Сценарий использования",
                code: r##"$ cargo run -- add "изучить владение" 2
✔ добавлена #1: изучить владение
$ cargo run -- add "написать API" 1
✔ добавлена #2: написать API
$ cargo run -- list
#1 [ ] изучить владение (p2)
#2 [ ] написать API (p1)
открытых: 2
$ cargo run -- done 1
✔ закрыта: изучить владение
$ cat tasks.json
{
  "next_id": 3,
  "tasks": [
    { "id": 1, "text": "изучить владение", "prio": 2, "done": true },
    { "id": 2, "text": "написать API", "prio": 1, "done": false }
  ]
}"##,
                runnable: false,
            },
            Block::Note("Расширения для самостоятельной работы: сортировка по приоритету в list (sort_by_key), дедлайны (chrono), цветной вывод (owo-colors), интерактивный выбор (dialoguer), полноценный парсер аргументов (clap)."),
            Block::Note("Тесты для проекта: Store::add и mark_done покрываются unit-тестами без файловой системы; storage::save/load — интеграционным тестом во временном каталоге (tempfile)."),
        ],
        quiz: vec![
            Question {
                text: "Что делает std::env::args().skip(1)?",
                options: &[
                    "Пропускает имя самой программы, оставляя аргументы",
                    "Удаляет первый аргумент навсегда",
                    "Читает переменные окружения",
                ],
                answer: 0,
                explain: "Первый элемент args — путь к бинарнику; для CLI он не нужен, поэтому skip(1).",
            },
            Question {
                text: "Почему unwrap_or_default() уместен при загрузке JSON?",
                options: &[
                    "Потому что ошибки не бывает",
                    "Битый или отсутствующий файл — нормальный сценарий для первого запуска; начинаем с пустого хранилища",
                    "Это быстрее",
                ],
                answer: 1,
                explain: "Первый запуск файла нет, после сбоя файл может быть повреждён — дефолтный Store корректная стратегия здесь.",
            },
        ],
        exercise: Some(Exercise {
            task: "Соберите CLI целиком по уроку и выполните сценарий: add двух задач, list, done первой, list. Убедитесь, что tasks.json обновляется.",
            starter: r##"# Соберите проект из двух предыдущих листингов:
# 1) models.rs  2) storage.rs  3) cli.rs  4) main.rs
# Затем:
# cargo run -- add "задача 1"
# cargo run -- add "задача 2" 2
# cargo run -- list
# cargo run -- done 1
# cargo run -- list"##,
            solution: r##"// Готово? У вас работающее CLI-приложение на Rust!
// Дальше: превратим тот же домен в REST API (следующий урок).
// Бонус: добавьте команду rm через Store::remove:
//   pub fn remove(&mut self, id: u32) -> Result<Task, String> {
//       let pos = self.tasks.iter().position(|t| t.id == id)
//           .ok_or_else(|| format!("задача {id} не найдена"))?;
//       Ok(self.tasks.remove(pos))
//   }"##,
        }),
    }
}

fn api_project() -> Lesson {
    Lesson {
        id: "project-api",
        title: "Проект 2: REST API на Axum",
        blocks: vec![
            Block::P("Финальный аккорд: тот же домен задач превращаем в HTTP-сервис. Axum — современный веб-фреймворк от команды Tokio: типобезопасные хендлеры, экстракторы запросов, встроенная интеграция с async."),
            Block::Heading("Схема API"),
            Block::List(&[
                "GET /tasks — список задач (JSON).",
                "POST /tasks {\"text\": \"...\", \"prio\": 1} — создать.",
                "PATCH /tasks/{id}/done — закрыть.",
                "DELETE /tasks/{id} — удалить.",
                "Хранилище — Arc<Mutex<Store>> (или DashMap/RwLock на проде).",
            ]),
            Block::Code {
                caption: "Cargo.toml и скелет сервера",
                code: r##"[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
"##,
                runnable: false,
            },
            Block::Code {
                caption: "src/main.rs — маршруты и хендлеры",
                code: r##"use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    text: String,
    prio: u8,
    done: bool,
}

#[derive(Default)]
struct Store {
    next_id: u32,
    tasks: Vec<Task>,
}

type Shared = Arc<Mutex<Store>>;

#[derive(Deserialize)]
struct NewTask {
    text: String,
    prio: Option<u8>,
}

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(Store::default()));

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}/done", patch(mark_done))
        .route("/tasks/{id}", delete(delete_task))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("API: http://127.0.0.1:3000/tasks");
    axum::serve(listener, app).await.unwrap();
}

async fn list_tasks(State(store): State<Shared>) -> Json<Vec<Task>> {
    Json(store.lock().unwrap().tasks.clone())
}

async fn create_task(
    State(store): State<Shared>,
    Json(body): Json<NewTask>,
) -> (StatusCode, Json<Task>) {
    let mut s = store.lock().unwrap();
    let task = Task {
        id: s.next_id,
        text: body.text,
        prio: body.prio.unwrap_or(1),
        done: false,
    };
    s.next_id += 1;
    s.tasks.push(task.clone());
    (StatusCode::CREATED, Json(task))
}

async fn mark_done(
    State(store): State<Shared>,
    Path(id): Path<u32>,
) -> StatusCode {
    let mut s = store.lock().unwrap();
    match s.tasks.iter_mut().find(|t| t.id == id) {
        Some(t) => {
            t.done = true;
            StatusCode::OK
        }
        None => StatusCode::NOT_FOUND,
    }
}

async fn delete_task(
    State(store): State<Shared>,
    Path(id): Path<u32>,
) -> StatusCode {
    let mut s = store.lock().unwrap();
    let before = s.tasks.len();
    s.tasks.retain(|t| t.id != id);
    if s.tasks.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}"##,
                runnable: false,
            },
            Block::Heading("Проверка: curl"),
            Block::Code {
                caption: "Сценарий тестирования API",
                code: r##"$ cargo run
$ curl http://127.0.0.1:3000/tasks
[]
$ curl -X POST http://127.0.0.1:3000/tasks \
    -H "Content-Type: application/json" \
    -d '{"text":"освоить axum","prio":2}'
{"id":0,"text":"освоить axum","prio":2,"done":false}
$ curl -X PATCH http://127.0.0.1:3000/tasks/0/done
$ curl http://127.0.0.1:3000/tasks
[{"id":0,"text":"освоить axum","prio":2,"done":true}]"##,
                runnable: false,
            },
            Block::Note("Экстракторы: State<T> — доступ к общим данным, Json<T> — разбор тела, Path — параметры URL. Возвращаемый тип описывает ответ: Json<T> сериализуется автоматически, StatusCode — код ответа."),
            Block::Note("Куда расти: постоянное хранилище (sqlx + PostgreSQL/SQLite), валидация (validator), обработка ошибок с thiserror + IntoResponse, тесты API (tower::ServiceExt::oneshot), логирование (tracing), Docker-образ. Rust + Axum — production-grade стек."),
        ],
        quiz: vec![
            Question {
                text: "Что делает экстрактор Json<NewTask> в хендлере?",
                options: &[
                    "Сериализует ответ",
                    "Разбирает тело запроса в структуру NewTask или возвращает 422",
                    "Логирует запрос",
                ],
                answer: 1,
                explain: "Экстракторы разбирают части запроса. Несовпадающее тело — автоматически ошибка 422 Unprocessable Entity.",
            },
            Question {
                text: "Почему хранилище — Arc<Mutex<Store>>?",
                options: &[
                    "Для скорости",
                    "Состояние общее между хендлерами; axum требует Send + Sync, мутация — через Mutex",
                    "Так требует serde",
                ],
                answer: 1,
                explain: "Общее состояние в axum передаётся в with_state и должно быть клонируемым и потокобезопасным. Arc+Mutex — каноничное решение.",
            },
        ],
        exercise: Some(Exercise {
            task: "Соберите API из урока, запустите и пройдите сценарий curl: создать две задачи, закрыть одну, удалить другую, проверить список. Бонус: добавьте хендлер GET /tasks/{id}, возвращающий одну задачу или 404.",
            starter: r##"// Добавьте к роутеру:
// .route("/tasks/{id}", get(get_task))
//
// И хендлер:
// async fn get_task(State(store): State<Shared>, Path(id): Path<u32>)
//     -> Result<Json<Task>, StatusCode> {
//     // TODO
//     Err(StatusCode::NOT_FOUND)
// }"##,
            solution: r##"use axum::{extract::{Path, State}, http::StatusCode, Json};

async fn get_task(
    State(store): State<Shared>,
    Path(id): Path<u32>,
) -> Result<Json<Task>, StatusCode> {
    let s = store.lock().unwrap();
    s.tasks
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// Result<Json<Task>, StatusCode> -> axum сам превратит Err в 404,
// а Ok в 200 с JSON-телом. Идиоматично и без ручной сборки ответов."##,
        }),
    }
}
