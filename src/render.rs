//! HTML-рендеринг: сборка страниц из контента курса.
//! Никаких шаблонизаторов — чистый формат!() с обязательным экранированием.

use std::collections::HashMap;

use crate::content::{Block, Exercise, Lesson, Question, Section};

/// Готовый к раздаче курс: фрагменты страниц + навигация.
/// Layout (шапка с именем пользователя) применяется при каждом запросе.
#[derive(Clone)]
pub struct Catalog {
    /// id урока -> (title, фрагмент тела страницы)
    lessons: HashMap<String, (String, String)>,
    home_body: String,
    not_found_body: String,
}

impl Catalog {
    pub fn build(sections: Vec<Section>) -> Catalog {
        let mut lessons = HashMap::new();
        let mut order: Vec<(String, &str)> = Vec::new();
        for section in &sections {
            for lesson in &section.lessons {
                order.push((lesson.id.to_string(), lesson.title));
            }
        }
        for section in &sections {
            for lesson in &section.lessons {
                let idx = order.iter().position(|(id, _)| id == lesson.id).unwrap();
                let prev = if idx > 0 { Some(&order[idx - 1]) } else { None };
                let next = order.get(idx + 1);
                let body = render_lesson(&sections, section, lesson, idx + 1, prev, next, order.len());
                lessons.insert(lesson.id.to_string(), (lesson.title.to_string(), body));
            }
        }
        let home_body = render_home_body(&sections, order.len());
        let not_found_body = String::from(
            "<main class=\"content nf\"><h1>404</h1><p>Такого урока нет. Вернитесь на <a href=\"/\">главную</a>.</p></main>",
        );
        Catalog { lessons, home_body, not_found_body }
    }

    /// Полная страница урока с шапкой текущего пользователя.
    pub fn find(&self, id: &str, user: Option<&str>) -> Option<String> {
        self.lessons
            .get(id)
            .map(|(title, body)| layout(title, body, user))
    }

    /// Страница 404 с шапкой текущего пользователя.
    pub fn not_found_page(&self, user: Option<&str>) -> String {
        layout("Не найдено", &self.not_found_body, user)
    }
}

/// Раскладка страницы: шапка, тело, скрипты.
fn layout(page_title: &str, body: &str, user: Option<&str>) -> String {
    let auth_box = match user {
        Some(name) => format!(
            "<div class=\"user-box\"><span class=\"user-name\">👤 {}</span>\
             <form method=\"post\" action=\"/logout\"><button class=\"logout-btn\" title=\"Выйти\">Выйти</button></form></div>",
            escape(name),
        ),
        None => String::new(),
    };
    format!(
        r#"<!DOCTYPE html>
<html lang="ru">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title} — Rust Academy</title>
<link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🦀</text></svg>">
<link rel="stylesheet" href="/static/app.css">
</head>
<body data-auth="{auth}">
<header class="topbar">
  <a class="logo" href="/">🦀 <span>Rust Academy</span></a>
  <div class="topbar-progress"><span id="overall-label">прогресс</span><div class="bar"><div id="overall-bar"></div></div></div>
  {auth_box}
</header>
{body}
<script src="/static/app.js"></script>
</body>
</html>"#,
        title = escape(page_title),
        auth = if user.is_some() { "1" } else { "0" },
        auth_box = auth_box,
        body = body,
    )
}

fn sidebar(sections: &[Section], current: Option<&str>) -> String {
    let mut html = String::from("<nav class=\"sidebar\"><a class=\"side-home\" href=\"/\">📚 Все модули</a>");
    for s in sections {
        html.push_str(&format!(
            "<div class=\"side-module\"><div class=\"side-module-title\">{} {}</div>",
            s.emoji,
            escape(s.title)
        ));
        for l in &s.lessons {
            let cls = if current == Some(l.id) { "side-link current" } else { "side-link" };
            html.push_str(&format!(
                "<a class=\"{cls}\" data-lesson=\"{id}\" href=\"/lesson/{id}\"><span class=\"dot\"></span>{title}</a>",
                cls = cls,
                id = escape(l.id),
                title = escape(l.title),
            ));
        }
        html.push_str("</div>");
    }
    html.push_str("</nav>");
    html
}

fn render_block(block: &Block) -> String {
    match block {
        Block::Heading(t) => format!("<h2>{}</h2>", escape(t)),
        Block::P(t) => format!("<p>{}</p>", escape(t)),
        Block::Code { caption, code, runnable } => {
            let btn = if *runnable {
                "<button class=\"play-btn\" title=\"Открыть код на play.rust-lang.org\">▶ Playground</button>"
            } else {
                ""
            };
            let cap = if caption.is_empty() { "" } else { &format!("<div class=\"code-caption\">{}</div>", escape(caption)) };
            format!(
                "{cap}<div class=\"codebox\"><pre><code class=\"rust\">{code}</code>{btn}</div>",
                cap = cap,
                code = escape(code),
                btn = btn,
            )
        }
        Block::Output(t) => format!(
            "<div class=\"output\"><div class=\"code-caption\">Вывод</div><pre>{}</pre></div>",
            escape(t)
        ),
        Block::Note(t) => format!("<div class=\"callout note\"><b>💡 Заметка.</b> {}</div>", escape(t)),
        Block::Warn(t) => format!("<div class=\"callout warn\"><b>⚠️ Важно.</b> {}</div>", escape(t)),
        Block::List(items) => {
            let li: Vec<String> = items.iter().map(|i| format!("<li>{}</li>", escape(i))).collect();
            format!("<ul>{}</ul>", li.concat())
        }
        Block::Steps(items) => {
            let li: Vec<String> = items.iter().map(|i| format!("<li>{}</li>", escape(i))).collect();
            format!("<ol>{}</ol>", li.concat())
        }
    }
}

fn render_quiz(quiz: &[Question]) -> String {
    if quiz.is_empty() {
        return String::new();
    }
    let mut html = String::from("<section class=\"quiz\"><h2>✅ Проверь себя</h2>");
    for (qi, q) in quiz.iter().enumerate() {
        html.push_str(&format!(
            "<form class=\"quiz-q\" data-answer=\"{answer}\"><div class=\"quiz-text\">{qi}. {text}</div>",
            answer = q.answer,
            qi = qi + 1,
            text = escape(q.text),
        ));
        for (oi, opt) in q.options.iter().enumerate() {
            html.push_str(&format!(
                "<label class=\"quiz-opt\"><input type=\"radio\" name=\"q{qi}\" value=\"{oi}\"><span>{opt}</span></label>",
                qi = qi,
                oi = oi,
                opt = escape(opt),
            ));
        }
        html.push_str(&format!(
            "<button type=\"submit\" class=\"check-btn\">Проверить</button><div class=\"quiz-result\" hidden></div><div class=\"quiz-explain\" hidden>💬 {}</div></form>",
            escape(q.explain),
        ));
    }
    html.push_str("</section>");
    html
}

fn render_exercise(ex: &Exercise) -> String {
    format!(
        "<section class=\"exercise\"><h2>💻 Практика</h2><p>{task}</p>\
         <div class=\"codebox\"><pre><code class=\"rust\">{starter}</code></pre><span class=\"play-btn starter-btn\">▶ Playground</span></div>\
         <details class=\"solution\"><summary>👁 Показать решение</summary>\
         <div class=\"codebox\"><pre><code class=\"rust\">{solution}</code></pre><span class=\"play-btn solution-btn\">▶ Playground</span></div>\
         </details></section>",
        task = escape(ex.task),
        starter = escape(ex.starter),
        solution = escape(ex.solution),
    )
}

#[allow(clippy::too_many_arguments)]
fn render_lesson(
    sections: &[Section],
    section: &Section,
    lesson: &Lesson,
    n: usize,
    prev: Option<&(String, &str)>,
    next: Option<&(String, &str)>,
    total: usize,
) -> String {
    let blocks: Vec<String> = lesson.blocks.iter().map(render_block).collect();
    let prev_nav = match prev {
        Some((id, title)) => format!("<a class=\"nav-btn\" href=\"/lesson/{id}\">← {title}</a>", id = id, title = escape(title)),
        None => String::from("<span></span>"),
    };
    let next_nav = match next {
        Some((id, title)) => format!("<a class=\"nav-btn\" href=\"/lesson/{id}\">{title} →</a>", id = id, title = escape(title)),
        None => format!("<a class=\"nav-btn done\" href=\"/\">Вернуться к курсу →</a>"),
    };
    let done_btn = format!(
        "<button id=\"done-btn\" data-lesson=\"{id}\">✔ Отметить пройденным</button>",
        id = escape(lesson.id)
    );
    format!(
        "<div class=\"page\">{sidebar}\
         <main class=\"content\">\
         <div class=\"crumbs\"><a href=\"/\">Курс</a> / <a href=\"/\">{section}</a> / <span>{title}</span></div>\
         <article>\
         <h1>{title}</h1>\
         <div class=\"lesson-meta\">Модуль «{section}» · урок {n} из {total}</div>\
         {blocks}\
         {quiz}\
         {exercise}\
         </article>\
         <div class=\"lesson-actions\">{done_btn}</div>\
         <div class=\"pager\">{prev_nav}{next_nav}</div>\
         </main></div>",
        sidebar = sidebar(sections, Some(lesson.id)),
        section = escape(section.title),
        title = escape(lesson.title),
        n = n,
        total = total,
        blocks = blocks.concat(),
        quiz = render_quiz(&lesson.quiz),
        exercise = lesson.exercise.as_ref().map(render_exercise).unwrap_or_default(),
        done_btn = done_btn,
        prev_nav = prev_nav,
        next_nav = next_nav,
    )
}

/// Главная страница: карта курса с прогрессом.
pub fn render_home(catalog: &Catalog, user: Option<&str>) -> String {
    layout("Курс Rust с нуля", &catalog.home_body, user)
}

fn render_home_body(sections: &[Section], total: usize) -> String {
    let mut cards = String::new();
    for (si, s) in sections.iter().enumerate() {
        let lessons: Vec<String> = s
            .lessons
            .iter()
            .map(|l| {
                format!(
                    "<a class=\"lesson-link\" data-lesson=\"{id}\" href=\"/lesson/{id}\"><span class=\"dot\"></span>{title}</a>",
                    id = escape(l.id),
                    title = escape(l.title),
                )
            })
            .collect();
        cards.push_str(&format!(
            "<section class=\"module-card\" data-module=\"{si}\">\
             <div class=\"module-head\"><span class=\"module-emoji\">{emoji}</span>\
             <div><h2>{n}. {title}</h2><p class=\"blurb\">{blurb}</p></div>\
             <span class=\"module-progress\" data-total=\"{lt}\">0/{lt}</span></div>\
             <div class=\"module-lessons\">{lessons}</div>\
             </section>",
            si = si,
            emoji = s.emoji,
            n = si + 1,
            title = escape(s.title),
            blurb = escape(s.blurb),
            lt = s.lessons.len(),
            lessons = lessons.concat(),
        ));
    }
    format!(
        "<main class=\"home\">\
         <section class=\"hero\">\
         <h1>🦀 Rust Academy</h1>\
         <p>Полный интерактивный курс Rust: от переменных до async/await, макросов и unsafe. \
         Каждый урок — теория, примеры, тест и практика. Прогресс сохраняется в аккаунте.</p>\
         <div class=\"hero-stats\"><span id=\"hero-done\">0</span> / {total} уроков пройдено</div>\
         </section>\
         {cards}\
         <section class=\"module-card about\"><h2>🛠 Как учиться</h2>\
         <ol><li>Читайте урок и запускайте примеры — кнопка <b>▶ Playground</b> открывает код в песочнице play.rust-lang.org.</li>\
         <li>Отвечайте на вопросы теста — разбор появляется сразу.</li>\
         <li>Делайте практические задания локально: <code>cargo new practice</code>, вставляйте код, <code>cargo run</code>.</li>\
         <li>Модуль 11 — большой проект, который собирается по мере прохождения курса.</li></ol>\
         </section>\
         </main>",
        total = total,
        cards = cards,
    )
}

/// Страница входа.
pub fn render_login(error: Option<&str>) -> String {
    let error_html = match error {
        Some(msg) => format!("<div class=\"login-error\">{}</div>", escape(msg)),
        None => String::new(),
    };
    let body = format!(
        "<main class=\"login-wrap\">\
         <form class=\"login-card\" method=\"post\" action=\"/login\">\
         <div class=\"login-logo\">🦀</div>\
         <h1>Rust Academy</h1>\
         <p class=\"login-sub\">Войдите, чтобы синхронизировать прогресс между устройствами</p>\
         {error_html}\
         <label>Логин\
         <input name=\"username\" autocomplete=\"username\" required autofocus></label>\
         <label>Пароль\
         <input name=\"password\" type=\"password\" autocomplete=\"current-password\" required></label>\
         <button type=\"submit\" class=\"login-btn\">Войти</button>\
         </form>\
         </main>"
    );
    layout("Вход", &body, None)
}

/// Экранирование HTML — применяется ко ВСЕМУ пользовательскому контенту.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}
