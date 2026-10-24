//! Модуль 7: Функциональный Rust — замыкания, итераторы, модули.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "⚡",
        title: "Функциональный Rust",
        blurb: "Замыкания и итераторы — декларативная обработка данных, а также организация кода: модули и крейты.",
        lessons: vec![closures(), iterators(), modules()],
    }
}

fn closures() -> Lesson {
    Lesson {
        id: "func-closures",
        title: "Замыкания",
        blocks: vec![
            Block::P("Замыкание (closure) — анонимная функция, которая может захватывать переменные из окружения. Синтаксис: |args| body. Это основа колбэков, итераторов и асинхронного кода."),
            Block::Code {
                caption: "Синтаксис и захват окружения",
                code: r##"fn main() {
    // Полная форма с типами и возвратом
    let add = |a: i32, b: i32| -> i32 { a + b };

    // Краткая форма — типы выводятся
    let double = |x| x * 2;

    println!("{}", add(2, 3));       // 5
    println!("{}", double(21));      // 42

    // Захват переменной из окружения
    let factor = 10;
    let scale = |x| x * factor;      // заимствует factor
    println!("{}", scale(5));        // 50
    println!("{factor}");            // factor всё ещё доступна
}"##,
                runnable: true,
            },
            Block::Heading("Три способа захвата"),
            Block::Code {
                caption: "Fn, FnMut, FnOnce",
                code: r##"fn main() {
    let name = String::from("мир");

    // Fn — захват по неизменяемой ссылке (&T): можно звать много раз
    let greet = || println!("привет, {name}");
    greet();
    greet();
    println!("{name}"); // name жива

    // FnMut — захват по изменяемой ссылке (&mut T)
    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    increment();
    println!("счётчик: {count}"); // 2

    // FnOnce — захват по значению (move): можно позвать ОДИН раз
    let data = String::from("владение");
    let consume = move || println!("забрал {data}");
    consume();
    // consume(); // ОШИБКА: значение перемещено
}"##,
                runnable: false,
            },
            Block::List(&[
                "`Fn` — читает окружение, вызывается сколько угодно раз.",
                "`FnMut` — изменяет захваченное, тоже многократно.",
                "`FnOnce` — потребляет захваченное, одноразовое. move-замыкание захватывает по значению.",
                "Каждый Fn также является FnMut и FnOnce (иерархия возможностей).",
            ]),
            Block::Heading("Замыкания как аргументы"),
            Block::Code {
                caption: "Функции высшего порядка",
                code: r##"fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    let result = apply(|x| x * x, 5);          // 25
    println!("{result}");

    // Возврат замыкания: должен быть move, Box или impl Fn
    let make_adder = |n| move |x| x + n;
    let add5 = make_adder(5);
    println!("{}", add5(10)); // 15
}"##,
                runnable: true,
            },
            Block::Note("На практике тип замыкания почти всегда не пишут руками — используют impl Fn(..) -> .. или дженерики. Замыкания — уникальные анонимные типы, их нельзя назвать по имени."),
        ],
        quiz: vec![
            Question {
                text: "Что делает ключевое слово move перед замыканием?",
                options: &[
                    "Заставляет захватить переменные по значению (забрать владение)",
                    "Перемещает замыкание в кучу",
                    "Запрещает захват окружения",
                ],
                answer: 0,
                explain: "move — захват по значению: замыкание становится владельцем. Обычно нужно при передаче в другой поток или возврате.",
            },
            Question {
                text: "Какое замыкание можно вызвать только один раз?",
                options: &["Fn", "FnMut", "FnOnce"],
                answer: 2,
                explain: "FnOnce потребляет захваченные значения, поэтому вторым вызовом значение уже недоступно.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию make_counter() -> impl FnMut() -> u32, возвращающую замыкание-счётчик: каждый вызов увеличивает и возвращает внутреннее значение.",
            starter: r##"fn make_counter() -> impl FnMut() -> u32 {
    // TODO
    || 0
}

fn main() {
    let mut next = make_counter();
    println!("{}", next()); // 1
    println!("{}", next()); // 2
}"##,
            solution: r##"fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn main() {
    let mut next = make_counter();
    println!("{}", next()); // 1
    println!("{}", next()); // 2
}"##,
        }),
    }
}

fn iterators() -> Lesson {
    Lesson {
        id: "func-iterators",
        title: "Итераторы",
        blocks: vec![
            Block::P("Итератор — ленивая последовательность значений с методом next(). Комбинаторы map, filter, fold и другие позволяют обрабатывать данные декларативно, а компилятор превращает цепочки в столь же быстрый код, как ручной цикл."),
            Block::Code {
                caption: "Три кита: map, filter, collect",
                code: r##"fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    // Собрать удвоенные чётные числа
    let result: Vec<i32> = nums
        .iter()
        .filter(|&&n| n % 2 == 0)  // лениво: ничего не вычисляется
        .map(|&n| n * 2)           // тоже лениво
        .collect();                // только collect запускает вычисление

    println!("{result:?}"); // [4, 8]

    // collect работает с разными контейнерами
    let set: std::collections::HashSet<i32> = [1, 1, 2, 3].into_iter().collect();
    println!("{set:?}");
}"##,
                runnable: true,
            },
            Block::Warn("Забыли collect() — цепочка ничего не сделает: итераторы ленивы. Компилятор предупредит об unused result, но не всегда очевидно, куда он делся. Итератор потребляется: после collect/map исходный iter() уже использован."),
            Block::Heading("fold, sum, count и не только"),
            Block::Code {
                caption: "Свёртка и агрегация",
                code: r##"fn main() {
    let nums = [1, 2, 3, 4, 5];

    // fold: аккумулятор + элемент -> новый аккумулятор
    let sum = nums.iter().fold(0, |acc, &x| acc + x);
    println!("сумма: {sum}");

    let product = nums.iter().fold(1, |acc, &x| acc * x);
    println!("произведение: {product}");

    // Специализированные методы (быстрее fold)
    println!("sum: {}", nums.iter().sum::<i32>());
    println!("max: {:?}", nums.iter().max());
    println!("count: {}", nums.iter().count());
    println!("any >4: {}", nums.iter().any(|&x| x > 4));
    println!("all >0: {}", nums.iter().all(|&x| x > 0));
}"##,
                runnable: true,
            },
            Block::Heading("iter(), into_iter(), iter_mut()"),
            Block::Code {
                caption: "Что даёт и что забирает",
                code: r##"fn main() {
    let mut v = vec![1, 2, 3];

    for x in v.iter() {        // &T — только читаем
        print!("{x} ");
    }
    println!();

    for x in v.iter_mut() {    // &mut T — можем менять
        *x *= 10;
    }
    println!("{v:?}");

    for x in v.into_iter() {   // T — потребляет вектор!
        print!("{x} ");
    }
    println!();
    // println!("{v:?}"); // ОШИБКА: v перемещён
}"##,
                runnable: false,
            },
            Block::Note("Свой итератор — реализуйте трейт Iterator { type Item; fn next(&mut self) -> Option<Item> }. Все комбинаторы map/filter/fold станут доступны автоматически. Идиома: итераторы — самый элегантный способ работы с данными в Rust."),
            Block::Note("От противного: enumerate() даёт индекс, zip() объединяет две последовательности, take/skip обрезают, chain соединяет, flat_map разворачивает. Изучите их — они заменяют большинство ручных циклов."),
        ],
        quiz: vec![
            Question {
                text: "Когда выполняются вычисления в цепочке итераторов?",
                options: &[
                    "Сразу при создании chain",
                    "Только при потреблении: collect, sum, for и т.п.",
                    "В отдельном потоке",
                ],
                answer: 1,
                explain: "Итераторы ленивы: map/filter лишь строят описание. Реальная работа начинается при потреблении (collect/sum/fold/for).",
            },
            Question {
                text: "Чем отличается v.iter() от v.into_iter()?",
                options: &[
                    "Ничем",
                    "iter() даёт &T и сохраняет v; into_iter() потребляет v, отдавая T по значению",
                    "into_iter() медленнее",
                ],
                answer: 1,
                explain: "into_iter забирает владение: после цикла вектор использовать нельзя. iter лишь заимствует.",
            },
        ],
        exercise: Some(Exercise {
            task: "Используя одну цепочку итераторов, возьмите числа 1..=20, оставьте кратные 3, возведите в квадрат и просуммируйте. Напечатайте результат (кратные 3: 3,6,9,12,15,18 → 9+36+81+144+225+324 = 819).",
            starter: r##"fn main() {
    let total: i32 = (1..=20)
        // TODO: filter, map, sum
        .sum();
    println!("{total}");
}"##,
            solution: r##"fn main() {
    let total: i32 = (1..=20)
        .filter(|n| n % 3 == 0)
        .map(|n| n * n)
        .sum();
    println!("{total}"); // 819
}"##,
        }),
    }
}

fn modules() -> Lesson {
    Lesson {
        id: "func-modules",
        title: "Модули, крейты, workspaces",
        blocks: vec![
            Block::P("Rust организует код иерархически: крейт (crate) — единица компиляции и публикации; модуль (mod) — пространство имён внутри крейта; путь (path) — способ обращения к элементам."),
            Block::Code {
                caption: "Модули и видимость",
                code: r##"mod network {
    pub struct Server {
        pub port: u16,
        secret_key: String, // приватное поле
    }

    impl Server {
        pub fn new(port: u16) -> Self {
            Self { port, secret_key: String::from("xyz") }
        }
        pub fn describe(&self) -> String {
            format!("сервер на :{}", self.port)
        }
    }

    fn helper() {} // приватная функция модуля
}

fn main() {
    let s = network::Server::new(8080);
    println!("{}", s.describe());
    // s.secret_key       // ОШИБКА: приватно
    // network::helper(); // ОШИБКА: приватно
}"##,
                runnable: false,
            },
            Block::List(&[
                "`pub` делает элемент видимым снаружи модуля. По умолчанию всё приватно.",
                "`pub(crate)` — виден внутри крейта, но не снаружи. `pub(super)` — родителю.",
                "use network::Server — импорт; use crate::network — путь внутри крейта.",
                "mod network; + файл network.rs (или network/mod.rs) — код в отдельном файле.",
            ]),
            Block::Heading("Структура растущего проекта"),
            Block::Code {
                caption: "Раскладка файлов",
                code: r##"my_app/
├── Cargo.toml
├── src/
│   ├── main.rs        # бинарный крейт (точка входа)
│   ├── lib.rs         # библиотечный крейт (опционально)
│   ├── config.rs      # mod config;
│   └── api/
│       ├── mod.rs     # mod api; — папка модулей
│       ├── routes.rs
│       └── handlers.rs
└── tests/             # интеграционные тесты"##,
                runnable: false,
            },
            Block::Heading("Крейты и workspace"),
            Block::Code {
                caption: "use — импорт из стандартной библиотеки и зависимостей",
                code: r##"use std::collections::HashMap;   // из std
use std::io::{self, Write};      // несколько: self = сам модуль io
use rand::Rng;                   // из внешнего крейта rand

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 1);
    let secret: u32 = rand::random();
    io::stdout().flush().ok();
    println!("{map:?} {secret}");
}"##,
                runnable: false,
            },
            Block::Note("Workspace — несколько крейтов в одном репозитории с общим Cargo.lock: [workspace] members = [\"api\", \"core\", \"cli\"] в корневом Cargo.toml. Так строятся крупные проекты: общие зависимости, быстрые пересборки."),
            Block::Note("pub use — реэкспорт: удобный публичный API. В lib.rs часто пишут pub use crate::models::User;, чтобы пользователи писали use my_app::User;"),
        ],
        quiz: vec![
            Question {
                text: "Что видимо снаружи модуля по умолчанию?",
                options: &["Всё", "Ничего — нужен pub", "Только структуры"],
                answer: 1,
                explain: "Инкапсуляция по умолчанию: без pub элемент приватен. Публичный API задаётся явно.",
            },
            Question {
                text: "Чем крейт отличается от модуля?",
                options: &[
                    "Ничем, синонимы",
                    "Крейт — единица компиляции и публикации (библиотека или бинарник); модуль — пространство имён внутри крейта",
                    "Модуль крупнее крейта",
                ],
                answer: 1,
                explain: "Крейт = пакет кода на crates.io (или бинарник). Модули — внутренняя структура крейта.",
            },
        ],
        exercise: Some(Exercise {
            task: "Разбейте программу на модули: mod math с публичной функцией square и приватной helper. Вызовите square из main. Попробуйте вызвать helper и убедитесь, что компилятор не даст.",
            starter: r##"// TODO: объявите модуль math
fn main() {
    // TODO: вызовите math::square(7)
}"##,
            solution: r##"mod math {
    pub fn square(x: i32) -> i32 {
        helper(x, x)
    }

    fn helper(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    println!("{}", math::square(7)); // 49
    // math::helper(1, 2); // ОШИБКА: приватная функция
}"##,
        }),
    }
}
