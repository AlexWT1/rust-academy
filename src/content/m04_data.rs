//! Модуль 4: Структуры, перечисления, match, коллекции.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🧱",
        title: "Структуры и данные",
        blurb: "Свои типы: структуры и методы, перечисления и Option, мощный match и стандартные коллекции.",
        lessons: vec![structs(), enums(), matching(), collections()],
    }
}

fn structs() -> Lesson {
    Lesson {
        id: "data-struct",
        title: "Структуры и методы",
        blocks: vec![
            Block::P("Структура (struct) — способ собрать разнородные данные в один тип. Это основа моделирования любой предметной области в Rust."),
            Block::Code {
                caption: "Определение, создание, обновление",
                code: r##"struct User {
    name: String,
    age: u8,
    active: bool,
}

fn main() {
    // Структурный синтаксис создания
    let mut user = User {
        name: String::from("Мария"),
        age: 27,
        active: true,
    };

    user.age += 1;

    // Синтаксис обновления структуры: ..возьмёт остальные поля
    let admin = User {
        name: String::from("root"),
        ..user // age и active возьмутся из user (переместятся!)
    };

    println!("{} {} лет, активен: {}", admin.name, admin.age, admin.active);
}"##,
                runnable: false,
            },
            Block::Heading("Кортежные структуры и unit-структуры"),
            Block::Code {
                caption: "Разновидности struct",
                code: r##"struct Meters(f64);   // кортежная структура: новое имя для типа
struct Millimeters(f64);

struct Point; // unit-структура: без полей

fn main() {
    let distance = Meters(42.0);
    println!("{} м", distance.0); // доступ по индексу
}"##,
                runnable: false,
            },
            Block::Note("Кортежная структура — идиома newtype: даже если внутри f64, компилятор не даст перепутать метры и миллиметры. Это нулевая стоимость, чистая семантика."),
            Block::Heading("Методы: impl-блоки"),
            Block::Code {
                caption: "Методы и ассоциированные функции",
                code: r##"struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Ассоциированная функция (без self) — как конструктор.
    // Вызов: Rectangle::square(3.0)
    fn square(size: f64) -> Self {
        Self { width: size, height: size }
    }

    // Метод: &self = &Rectangle, только читает
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // &mut self — может менять поля
    fn scale(&mut self, k: f64) {
        self.width *= k;
        self.height *= k;
    }

    // self (без &) — забирает владение, превращая себя в другой тип
    fn into_pair(self) -> (f64, f64) {
        (self.width, self.height)
    }
}

fn main() {
    let mut rect = Rectangle::square(2.0);
    println!("площадь: {}", rect.area());
    rect.scale(1.5);
    println!("после масштабирования: {}", rect.area());
    let (w, h) = rect.into_pair();
    println!("{} x {h}", w);
}"##,
                runnable: true,
            },
            Block::List(&[
                "`&self` — метод только читает (самый частый).",
                "`&mut self` — метод изменяет объект.",
                "`self` — метод потребляет значение (typical: into_iter, into_pair).",
                "`Self` внутри impl — псевдоним типа самого impl.",
            ]),
            Block::Heading("Вывод на печать: Debug и Display"),
            Block::Code {
                caption: "derive(Debug) — быстрый вывод {:?}",
                code: r##"#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: -4 };
    println!("{:?}", p);       // Point { x: 3, y: -4 }
    println!("{:#?}", p);      // красивый многострочный вывод
    let q = p.clone();         // явное копирование
    println!("равны? {}", p == q);
}"##,
                runnable: true,
            },
            Block::Note("#[derive(Debug, Clone, PartialEq, Default, Hash...)] — макрос, автоматически реализующий стандартные трейты. Начинайте с Debug — без него не напечатать структуру через {:?}."),
        ],
        quiz: vec![
            Question {
                text: "Что означает &self в методе?",
                options: &[
                    "Метод забирает владение структурой",
                    "Метод неизменяемо заимствует структуру — только читает",
                    "Метод может менять поля структуры",
                ],
                answer: 1,
                explain: "&self — сокращение для self: &Self. Заимствование для чтения. Для мутации — &mut self, для потребления — self.",
            },
            Question {
                text: "Зачем нужен newtype-паттерн struct Meters(f64)?",
                options: &[
                    "Чтобы сэкономить память",
                    "Чтобы создать различимый тип: нельзя случайно перепутать метры с другими f64",
                    "Такие структуры быстрее",
                ],
                answer: 1,
                explain: "Newtype даёт типовою различимость с нулевой рантайм-стоимостью. Компилятор не даст сложить метры с секундами.",
            },
        ],
        exercise: Some(Exercise {
            task: "Создайте структуру Book { title: String, pages: u32 } с методами is_long(&self) -> bool (больше 500 страниц) и прочитайте поля. Добавьте derive(Debug) и напечатайте книгу через {:?}.",
            starter: r##"#[derive(Debug)]
struct Book {
    // TODO: поля
}

fn main() {
    // TODO: создайте книгу, напечатайте {:?} и is_long()
}"##,
            solution: r##"#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

impl Book {
    fn is_long(&self) -> bool {
        self.pages > 500
    }
}

fn main() {
    let book = Book {
        title: String::from("Программирование на Rust"),
        pages: 560,
    };
    println!("{:?}", book);
    println!("Длинная? {}", book.is_long());
}"##,
        }),
    }
}

fn enums() -> Lesson {
    Lesson {
        id: "data-enum",
        title: "Перечисления и Option",
        blocks: vec![
            Block::P("Enum в Rust сильнее, чем в других языках: варианты могут нести данные разных типов. Это алгебраические типы данных (sum types) — фундамент всей обработки ошибок."),
            Block::Code {
                caption: "Enum с данными в вариантах",
                code: r##"enum Shape {
    Circle(f64),                 // радиус
    Rectangle { w: f64, h: f64 },// именованные поля
    Triangle,                    // без данных
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14159 * r * r,
        Shape::Rectangle { w, h } => w * h,
        Shape::Triangle => 1.0,
    }
}

fn main() {
    let shapes = [Shape::Circle(1.0), Shape::Rectangle { w: 2.0, h: 3.0 }];
    for s in &shapes {
        println!("{:.2}", area(s));
    }
}"##,
                runnable: true,
            },
            Block::Heading("Option<T> — отсутствие значения"),
            Block::P("В Rust нет null. Вместо него — enum Option<T> со двумя вариантами: Some(value) и None. Компилятор заставляет обработать оба случая — ссылки, указывающие в никуда, невозможны."),
            Block::Code {
                caption: "Option в действии",
                code: r##"fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Анна")),
        2 => Some(String::from("Борис")),
        _ => None,
    }
}

fn main() {
    let found = find_user(1);
    // Работаем безопасно:
    match found {
        Some(name) => println!("Найден: {name}"),
        None => println!("Пользователь не найден"),
    }

    // Удобные методы Option:
    println!("{}", find_user(1).unwrap_or("гость".to_string()));
    println!("{}", find_user(9).unwrap_or("гость".to_string()));
    println!("{}", find_user(2).is_some());
}"##,
                runnable: true,
            },
            Block::Output("Найден: Анна\nАнна\nгость\ntrue"),
            Block::List(&[
                "unwrap_or(default) — значение или заглушка.",
                "map(|x| ...) — преобразование, если есть значение.",
                "and_then(|x| ...) — цепочка вычислений, которые могут вернуть None.",
                "unwrap() — достать значение или паника (только для прототипов и тестов!).",
                "if let Some(x) = opt { ... } — когда интересен только один вариант.",
            ]),
            Block::Code {
                caption: "if let — компактный match",
                code: r##"fn main() {
    let maybe: Option<i32> = Some(42);
    if let Some(n) = maybe {
        println!("получили {n}");
    } else {
        println!("пусто");
    }
}"##,
                runnable: true,
            },
            Block::Warn("Забудьте про null-проверки «на всякий случай». В Rust либо тип Option — и компилятор заставит обработать None, либо обычный T — и значение гарантированно есть. Это устраняет целый класс ошибок."),
        ],
        quiz: vec![
            Question {
                text: "Как в Rust представить «может быть строка, может быть ничего»?",
                options: &["&str с null", "Option<String>", "Empty String"],
                answer: 1,
                explain: "Option<T> — идиоматический способ. None и Some(T) — два варианта; обработать оба заставит match или методы Option.",
            },
            Question {
                text: "Что вернёт Some(5).unwrap_or(0)?",
                options: &["0", "5", "Панику"],
                answer: 1,
                explain: "unwrap_or возвращает внутреннее значение, если оно есть, и запасное — если None. Здесь есть 5.",
            },
        ],
        exercise: Some(Exercise {
            task: "Реализуйте enum StatusCode { Ok, NotFound, ServerError } с методом message(&self) -> &'static str. Через match верните описание каждого.",
            starter: r##"enum StatusCode {
    // TODO: варианты
}

impl StatusCode {
    fn message(&self) -> &'static str {
        // TODO: match
        ""
    }
}

fn main() {
    // TODO: напечатайте сообщения всех трёх статусов
}"##,
            solution: r##"enum StatusCode {
    Ok,
    NotFound,
    ServerError,
}

impl StatusCode {
    fn message(&self) -> &'static str {
        match self {
            StatusCode::Ok => "всё хорошо",
            StatusCode::NotFound => "не найдено",
            StatusCode::ServerError => "ошибка сервера",
        }
    }
}

fn main() {
    let codes = [StatusCode::Ok, StatusCode::NotFound, StatusCode::ServerError];
    for c in &codes {
        println!("{}", c.message());
    }
}"##,
        }),
    }
}

fn matching() -> Lesson {
    Lesson {
        id: "data-match",
        title: "Match и паттерны",
        blocks: vec![
            Block::P("match — не просто switch. Это мощный механизм разбора структуры данных: паттерны умеют связывать внутренности вариантов, проверять диапазоны, добавлять условия (guards) и деструктурировать."),
            Block::Code {
                caption: "Виды паттернов",
                code: r##"enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("выход"),
        Message::Move { x, y } => {
            // деструктуризация именованных полей
            println!("движение в ({x}, {y})");
        }
        Message::Write(text) if text.len() > 10 => {
            // guard: дополнительное условие
            println!("длинное сообщение: {text}");
        }
        Message::Write(text) => println!("пишем: {text}"),
        Message::ChangeColor(r, g, b) => println!("цвет rgb({r},{g},{b})"),
    }
}

fn main() {
    process(Message::Move { x: 3, y: -2 });
    process(Message::Write(String::from("привет")));
}"##,
                runnable: true,
            },
            Block::Heading("Связывание в других контекстах"),
            Block::Code {
                caption: "let тоже использует паттерны",
                code: r##"fn main() {
    // Деструктуризация кортежа
    let (a, b, c) = (1, "два", 3.0);

    // Игнорирование: _
    let (x, _) = (10, 20);

    // Диапазоны и multiple
    let n = 5;
    match n {
        1 | 2 | 3 => println!("маленькое"),
        4..=6 => println!("среднее"),
        _ => println!("большое"),
    }

    // Вложенные паттерны: Option внутри Result
    let maybe: Option<Result<i32, String>> = Some(Ok(7));
    match maybe {
        Some(Ok(v)) => println!("успех: {v}"),
        Some(Err(e)) => println!("ошибка: {e}"),
        None => println!("нет значения"),
    }
    let _ = (a, b, c, x);
}"##,
                runnable: true,
            },
            Block::Heading("while let и удобные хелперы"),
            Block::Code {
                caption: "while let — цикл, пока паттерн совпадает",
                code: r##"fn main() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("сняли {top}");
    }
    // цикл закончится, когда pop() вернёт None
}"##,
                runnable: true,
            },
            Block::Warn("match должен быть исчерпывающим: пропустили вариант — ошибка компиляции. Это значит: добавление нового варианта в enum ломает ВСЕ места, где он обрабатывается, и компилятор покажет каждое. Рефакторинг в Rust безопасен."),
            Block::Note("Сравнение с if: когда вариантов больше двух-трёх и они структурные — выбирайте match. Для простых условий if читабельнее."),
        ],
        quiz: vec![
            Question {
                text: "Что произойдёт, если добавить новый вариант в enum, который разбирается в нескольких match?",
                options: &[
                    "Ничего, match всё стерпит",
                    "Каждый неохватывающий match даст ошибку компиляции",
                    "Ошибка появится в рантайме",
                ],
                answer: 1,
                explain: "Исчерпывающий match — одна из суперспособностей Rust: компилятор укажет все места, требующие обновления.",
            },
            Question {
                text: "Что делает паттерн-гард: Match::Write(t) if t.len() > 10 => ...?",
                options: &[
                    "Ограничивает совпадение дополнительным условием",
                    "Создаёт новый вариант",
                    "Игнорирует значение t",
                ],
                answer: 0,
                explain: "Гард (if после паттерна) добавляет условие; если оно ложно, match ищет следующую ветку.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию, принимающую Result<Option<i32>, String> и печатающую: значение, ошибку или отсутствие значения. Протестируйте все три случая.",
            starter: r##"fn describe(r: Result<Option<i32>, String>) {
    // TODO: match с вложенными паттернами
}

fn main() {
    describe(Ok(Some(5)));
    describe(Ok(None));
    describe(Err("сбой".to_string()));
}"##,
            solution: r##"fn describe(r: Result<Option<i32>, String>) {
    match r {
        Ok(Some(v)) => println!("значение: {v}"),
        Ok(None) => println!("значения нет"),
        Err(e) => println!("ошибка: {e}"),
    }
}

fn main() {
    describe(Ok(Some(5)));
    describe(Ok(None));
    describe(Err("сбой".to_string()));
}"##,
        }),
    }
}

fn collections() -> Lesson {
    Lesson {
        id: "data-collections",
        title: "Коллекции: Vec, String, HashMap",
        blocks: vec![
            Block::P("Три коллекции используются в 95% кода: вектор Vec<T> — динамический массив, String — владеющая строка, HashMap<K, V> — словарь. Все они хранят данные в куче и растут по мере необходимости."),
            Block::Heading("Vec<T>"),
            Block::Code {
                caption: "Вектор: создание, добавление, доступ",
                code: r##"fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // макрос vec! — сокращение
    let v2 = vec![10, 20, 30];

    // Доступ: индекс (паника вне границ) или get (Option)
    println!("{}", v[0]);               // 1
    println!("{:?}", v.get(5));         // None — безопасно
    println!("{}", v2[1]);              // 20

    // Обход
    for n in &v {
        print!("{n} ");
    }
    println!();

    // Мутация во время обхода — через &mut
    for n in &mut v {
        *n *= 10;
    }
    println!("{v:?}"); // [10, 20, 30]
}"##,
                runnable: true,
            },
            Block::Warn("v[i] при выходе за границы — паника. v.get(i) возвращает Option<&T> — обрабатывайте None. Выбирайте get, когда границы зависят от данных."),
            Block::Heading("String"),
            Block::Code {
                caption: "String — владеющая, растущая строка",
                code: r##"fn main() {
    let mut s = String::from("Привет");
    s.push_str(", ");
    s.push('м');
    s.push_str("ир");
    println!("{s}"); // Привет, мир

    // Конкатенация: + забирает владение правой части
    let a = String::from("hello ");
    let b = String::from("rust");
    let c = a + &b;        // a перемещён!
    // println!("{a}");    // ОШИБКА: a больше не валиден
    println!("{c}");

    // format! — не забирает владение, самый удобный
    let d = format!("{c} {b}!");
    println!("{d}");

    // Длина в байтах UTF-8, НЕ в символах!
    println!("{}", "привет".len()); // 12 байт
}"##,
                runnable: true,
            },
            Block::Note("String — это обёртка над Vec<u8>, гарантирующая валидный UTF-8. Индексация по символам отсутствует (символы разной длины в байтах). Для посимвольного обхода: s.chars()."),
            Block::Heading("HashMap<K, V>"),
            Block::Code {
                caption: "Словарь",
                code: r##"use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Анна".to_string(), 95);
    scores.insert("Борис".to_string(), 88);

    // Чтение: get -> Option<&V>
    if let Some(score) = scores.get("Анна") {
        println!("Анна: {score}");
    }

    // entry — вставить или обновить за один шаг
    scores.entry("Борис".to_string()).and_modify(|s| *s += 10);
    scores.entry("Вера".to_string()).or_insert(70);
    println!("{scores:?}");

    // Классика: подсчёт частот слов
    let text = "the quick the fox the";
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("{counts:?}"); // {"the": 3, "quick": 1, "fox": 1}
}"##,
                runnable: true,
            },
            Block::Note("HashMap из std не упорядочен. Если нужен порядок вставки — indexmap (внешний крейт). Если сортированный по ключам — BTreeMap."),
        ],
        quiz: vec![
            Question {
                text: "Что безопаснее для доступа к элементу вектора с возможной ошибкой?",
                options: &["v[i] — паникует вне границ", "v.get(i) — возвращает Option", "Оба одинаково безопасны"],
                answer: 1,
                explain: "get возвращает Option<&T>: Some(&x) или None. Индексирование удобно, когда границы гарантированы.",
            },
            Question {
                text: "Сколько байт займёт \"аб\".len()?",
                options: &["2 — по числу символов", "4 — каждая кириллическая буква 2 байта в UTF-8", "1"],
                answer: 1,
                explain: "len() возвращает длину в байтах. Кириллица в UTF-8 занимает по 2 байта на символ.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию, которая принимает список слов и возвращает Vec<String> — слова, начинающиеся с заглавной буквы. Используйте chars().next() и filter.",
            starter: r##"fn capitalized(words: &[&str]) -> Vec<String> {
    // TODO
    vec![]
}

fn main() {
    let words = ["Rust", "rust", "Язык", "язык", "C"];
    println!("{:?}", capitalized(&words)); // ["Rust", "Язык", "C"]
}"##,
            solution: r##"fn capitalized(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .filter(|w| w.chars().next().map_or(false, |c| c.is_uppercase()))
        .map(|w| w.to_string())
        .collect()
}

fn main() {
    let words = ["Rust", "rust", "Язык", "язык", "C"];
    println!("{:?}", capitalized(&words));
}"##,
        }),
    }
}
