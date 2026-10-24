//! Контент курса: каждый модуль курса — отдельный файл в этой папке.
//! Данные — чистые статические структуры, рендерятся в HTML движком `render`.

mod m01_hello;
mod m02_basics;
mod m03_ownership;
mod m04_data;
mod m05_errors;
mod m06_generics;
mod m07_func;
mod m08_smart;
mod m09_concurrency;
mod m10_advanced;
mod m11_project;

/// Блок урока — элемент тела страницы.
pub enum Block {
    /// Подзаголовок внутри урока.
    Heading(&'static str),
    /// Абзац текста.
    P(&'static str),
    /// Код. `runnable = true` добавляет кнопку «Открыть в Playground».
    Code {
        caption: &'static str,
        code: &'static str,
        runnable: bool,
    },
    /// Вывод программы.
    Output(&'static str),
    /// Полезное замечание.
    Note(&'static str),
    /// Важное предупреждение (частая ошибка).
    Warn(&'static str),
    /// Маркированный список.
    List(&'static [&'static str]),
    /// Нумерованный список (шаги).
    Steps(&'static [&'static str]),
}

/// Вопрос теста. `answer` — индекс правильного варианта.
pub struct Question {
    pub text: &'static str,
    pub options: &'static [&'static str],
    pub answer: usize,
    pub explain: &'static str,
}

/// Практическое задание к уроку.
pub struct Exercise {
    pub task: &'static str,
    pub starter: &'static str,
    pub solution: &'static str,
}

pub struct Lesson {
    /// Уникальный идентификатор, используется в URL: /lesson/{id}
    pub id: &'static str,
    pub title: &'static str,
    pub blocks: Vec<Block>,
    pub quiz: Vec<Question>,
    pub exercise: Option<Exercise>,
}

pub struct Section {
    pub emoji: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
    pub lessons: Vec<Lesson>,
}

/// Весь курс целиком. Порядок = порядок отображения.
pub fn course() -> Vec<Section> {
    vec![
        m01_hello::section(),
        m02_basics::section(),
        m03_ownership::section(),
        m04_data::section(),
        m05_errors::section(),
        m06_generics::section(),
        m07_func::section(),
        m08_smart::section(),
        m09_concurrency::section(),
        m10_advanced::section(),
        m11_project::section(),
    ]
}
