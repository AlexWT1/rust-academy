//! Модуль 6: Дженерики, трейты, времена жизни.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🧩",
        title: "Дженерики и трейты",
        blurb: "Обобщённый код без потери скорости, полиморфизм через трейты и времена жизни ссылок.",
        lessons: vec![generics(), traits(), lifetimes()],
    }
}

fn generics() -> Lesson {
    Lesson {
        id: "gen-types",
        title: "Дженерики",
        blocks: vec![
            Block::P("Дженерики позволяют писать код, работающий с разными типами, без дублирования. Компилятор генерирует специализированную версию для каждого использованного типа (мономорфизация) — абстракция бесплатна в рантайме."),
            Block::Code {
                caption: "Обобщённая функция и структура",
                code: r##"// T — параметр типа, используется как обычный тип
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
    fn swap(self) -> Self {
        Self { first: self.second, second: self.first }
    }
}

fn main() {
    println!("{}", largest(&[3, 7, 2]));          // 7
    println!("{}", largest(&['а', 'я', 'б']));    // я

    let p = Pair::new(1, 2).swap();
    println!("{} {}", p.first, p.second); // 2 1
}"##,
                runnable: true,
            },
            Block::Note("Обратите внимание на PartialOrd — ограничение (trait bound): мы требуем от T умение сравниваться. Дженерики без ограничений бесполезны: с T нельзя ничего сделать."),
            Block::Heading("Ограничения (trait bounds)"),
            Block::Code {
                caption: "Три способа записать ограничение",
                code: r##"use std::fmt::Debug;

// 1. В угловых скобках
fn show<T: Debug>(x: T) {
    println!("{x:?}");
}

// 2. where-клауза — читается лучше при сложных сигнатурах
fn process<T, U>(t: T, u: U) -> String
where
    T: Debug + Clone,
    U: Into<String>,
{
    let copy = t.clone();
    format!("{copy:?} + {}", u.into())
}

// 3. impl Trait — для одного параметра, лаконично
fn print_all(items: impl IntoIterator<Item = i32>) {
    for i in items {
        print!("{i} ");
    }
    println!();
}

fn main() {
    show(42);
    show(String::from("текст"));
    println!("{}", process(1, "два"));
    print_all([1, 2, 3]);
}"##,
                runnable: true,
            },
            Block::Heading("Дженерики в стандартной библиотеке — повсюду"),
            Block::Code {
                caption: "Option<T>, Result<T, E>, Vec<T>, HashMap<K, V>",
                code: r##"fn main() {
    // Один и тот же тип Option работает с любыми T
    let int_opt: Option<i32> = Some(5);
    let str_opt: Option<String> = None;

    let v: Vec<&str> = vec!["a", "b"];
    let mut m: std::collections::HashMap<char, u32> = std::collections::HashMap::new();
    m.insert('x', 24);

    println!("{int_opt:?} {str_opt:?} {v:?} {m:?}");
}"##,
                runnable: true,
            },
            Block::Note("Box<T>, Rc<T> и другие умные указатели тоже дженерики — увидим их позже. Скорость: из-за мономорфизации Vec<i32> и Vec<String> — это два разных конкретных типа, без рантайм-диспетчеризации."),
        ],
        quiz: vec![
            Question {
                text: "Что такое мономорфизация?",
                options: &[
                    "Компилятор генерирует отдельную версию функции для каждого конкретного типа",
                    "Ограничение: один тип на весь проект",
                    "Особый вид ошибки компиляции",
                ],
                answer: 0,
                explain: "Мономорфизация даёт нулевую стоимость абстракций: generic-код разворачивается в конкретный при компиляции.",
            },
            Question {
                text: "Зачем нужны trait bounds вроде <T: PartialOrd>?",
                options: &[
                    "Для красоты",
                    "Чтобы сообщить компилятору, какие операции доступны у T",
                    "Чтобы ускорить компиляцию",
                ],
                answer: 1,
                explain: "Без ограничения у T нет никаких гарантированных операций. Bounds описывают возможности типа.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите обобщённую функцию second<T>(items: &[T]) -> Option<&T>, возвращающую второй элемент. Протестируйте на векторе чисел и строках.",
            starter: r##"fn second<T>(items: &[T]) -> Option<&T> {
    // TODO
    None
}

fn main() {
    println!("{:?}", second(&[1, 2, 3]));
    println!("{:?}", second(&["a"]));
}"##,
            solution: r##"fn second<T>(items: &[T]) -> Option<&T> {
    items.get(1)
}

fn main() {
    println!("{:?}", second(&[1, 2, 3])); // Some(2)
    println!("{:?}", second(&["a"]));     // None
}"##,
        }),
    }
}

fn traits() -> Lesson {
    Lesson {
        id: "gen-traits",
        title: "Трейты",
        blocks: vec![
            Block::P("Трейт — интерфейс: набор методов, который тип обещает реализовать. Это главный механизм полиморфизма в Rust — аналог интерфейсов, но с реализациями по умолчанию и возможностью имплементации для чужих типов."),
            Block::Code {
                caption: "Трейт и реализации",
                code: r##"trait Summary {
    fn author(&self) -> String;          // обязательный метод

    fn preview(&self) -> String {         // реализация по умолчанию
        format!("(автор: {})", self.author())
    }
}

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    author: String,
    text: String,
}

impl Summary for Article {
    fn author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.author)
    }
    // preview наследуем по умолчанию
}

fn main() {
    let a = Article { title: String::from("Rust 2030"), author: String::from("Мария") };
    let t = Tweet { author: String::from("dev"), text: String::from("hello!") };
    println!("{}", a.preview());
    println!("{}", t.preview());
}"##,
                runnable: true,
            },
            Block::Heading("Трейт-объекты: динамическая диспетчеризация"),
            Block::Code {
                caption: "&dyn Trait — разные типы в одной коллекции",
                code: r##"trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> String { "гав".into() }
}
impl Animal for Cat {
    fn speak(&self) -> String { "мяу".into() }
}

fn main() {
    // Vec<Box<dyn Animal>> — heterogeneous collection
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in &zoo {
        println!("{}", animal.speak());
    }
}"##,
                runnable: true,
            },
            Block::List(&[
                "Дженерики (<T: Trait>) — статическая диспетчеризация: для каждого типа свой код, быстро, но больше бинарник.",
                "Трейт-объекты (dyn Trait) — динамическая диспетчеризация через таблицу vtable: гибко (heterogeneous), небольшой рантайм-накладной.",
                "Правило безопасности объектов: трейт с методами-обобщёнками не может быть dyn (object safety).",
            ]),
            Block::Heading("Полезные трейты стандартной библиотеки"),
            Block::Code {
                caption: "Debug, Clone, Default, From",
                code: r##"#[derive(Debug, Clone, PartialEq, Default)]
struct Config {
    host: String,
    port: u16,
}

fn main() {
    let c1 = Config::default();
    let c2 = c1.clone();
    println!("{c1:?} == {c2:?}: {}", c1 == c2);

    // From/Into — преобразования типов
    let port: u32 = 8080;
    let port64: u64 = port.into(); // u32: Into<u64>
    println!("{port64}");
}"##,
                runnable: true,
            },
            Block::Note("Трейты автоматически реализуются при derive: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default. Правило одиночки (orphan rule): реализовать чужой трейт можно только для своего типа (или свой трейт — для чужого типа)."),
        ],
        quiz: vec![
            Question {
                text: "Чем dyn Trait отличается от <T: Trait>?",
                options: &[
                    "dyn Trait — динамическая диспетчеризация через vtable; generic — статическая, с генерацией кода под каждый тип",
                    "Ничем",
                    "dyn Trait быстрее всегда",
                ],
                answer: 0,
                explain: "dyn — гибкость и один код; generic — скорость и specialization. Выбор зависит от сценария.",
            },
            Question {
                text: "Можно ли реализовать трейт Display из std для типа из std, например Vec<String>?",
                options: &[
                    "Да, всегда",
                    "Нет — правило одиночки (orphan rule): либо трейт, либо тип должны быть вашими",
                    "Только с unsafe",
                ],
                answer: 1,
                explain: "Orphan rule предотвращает конфликты реализаций. Обходной путь — newtype-обёртка struct MyVec(Vec<String>).",
            },
        ],
        exercise: Some(Exercise {
            task: "Создайте трейт Area { fn area(&self) -> f64 } и реализуйте его для Circle и Square. Напишите функцию total_area(items: &[Box<dyn Area>]) -> f64, суммирующую площади.",
            starter: r##"trait Area {
    // TODO
}

// TODO: Circle { radius: f64 }, Square { side: f64 }

fn main() {
    // TODO: посчитайте суммарную площадь круга r=1 и квадрата a=2
}"##,
            solution: r##"trait Area {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Area for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

impl Area for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

fn total_area(items: &[Box<dyn Area>]) -> f64 {
    items.iter().map(|s| s.area()).sum()
}

fn main() {
    let shapes: Vec<Box<dyn Area>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    println!("{:.2}", total_area(&shapes));
}"##,
        }),
    }
}

fn lifetimes() -> Lesson {
    Lesson {
        id: "gen-lifetimes",
        title: "Времена жизни",
        blocks: vec![
            Block::P("Время жизни (lifetime) — область, в которой ссылка действительна. Обычно компилятор выводит его сам, но когда функция возвращает ссылку, отношения между входными и выходными ссылками нужно назвать явно."),
            Block::Code {
                caption: "Зачем нужны аннотации",
                code: r##"// Компилятор не знает: 'a относится к x, к y или к обоим?
// fn longest(x: &str, y: &str) -> &str { ... } // ОШИБКА

// 'a — договорённость: результат живёт не дольше,
// чем оба входных аргумента
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("длинная строка");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("{result}"); // ок: s2 жива
    }
    // println!("{result}"); // ОШИБКА: s2 умерла, а result на неё ссылался
}"##,
                runnable: false,
            },
            Block::List(&[
                "Аннотации НЕ меняют время жизни — только описывают связи для компилятора.",
                "'a в двух аргументах значит: оба аргумента живут как минимум 'a.",
                "Правило элиминации: каждая ссылка-параметр получает свой lifetime; если параметров-ссылок несколько, а один — &self, его lifetime достаётся результату.",
            ]),
            Block::Code {
                caption: "Времена жизни в структурах",
                code: r##"struct Excerpt<'a> {
    part: &'a str, // структура держит заимствование
}

impl<'a> Excerpt<'a> {
    fn announce(&self) -> &str {
        self.part
    }
}

fn main() {
    let novel = String::from("Давным-давно. Жил-был программист.");
    let excerpt = Excerpt { part: &novel[..15] };
    println!("{}", excerpt.announce());
}"##,
                runnable: true,
            },
            Block::Note("'static — самый долгий lifetime: живёт всю программу. Строковые литералы (&'static str) и Box::leak имеют его. Увидите 'static в трейтах и замыканиях — это про «ссылка живёт вечно»."),
            Block::Warn("Не добавляйте lifetime-аннотации «на всякий случай»: если код компилируется без них — не пишите их. Явные 'a нужны только там, где компилятор не может вывести связи сам."),
        ],
        quiz: vec![
            Question {
                text: "Меняют ли аннотации 'a реальные времена жизни?",
                options: &["Да, продлевают", "Нет, только описывают связи ссылок для проверки", "Укорачивают"],
                answer: 1,
                explain: "Аннотации — декларативный контракт. Компилятор проверяет соответствие, но ничего не изменяет.",
            },
            Question {
                text: "Что означает &'static str?",
                options: &[
                    "Строка, которая никогда не изменится",
                    "Ссылка, действительная всю работу программы (например, литерал)",
                    "Строка в куче",
                ],
                answer: 1,
                explain: "'static живёт всю программу. Строковые литералы компилируются в сегмент данных бинарника, поэтому их ссылки живут вечно.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию first_longest<'a>(words: &'a [&'a str]) -> Option<&'a str>, возвращающую самое длинное слово. Она должна компилироваться с аннотацией 'a.",
            starter: r##"fn first_longest(words: &[&str]) -> Option<&str> {
    // TODO: max_by_key по длине
    None
}

fn main() {
    let words = ["один", "двадцать", "три"];
    println!("{:?}", first_longest(&words));
}"##,
            solution: r##"fn first_longest<'a>(words: &'a [&'a str]) -> Option<&'a str> {
    words.iter().copied().max_by_key(|w| w.len())
}

fn main() {
    let words = ["один", "двадцать", "три"];
    println!("{:?}", first_longest(&words)); // Some("двадцать")
}"##,
        }),
    }
}
