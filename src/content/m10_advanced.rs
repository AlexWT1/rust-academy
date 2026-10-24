//! Модуль 10: Продвинутое — макросы, unsafe, тестирование.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🧪",
        title: "Продвинутый Rust",
        blurb: "Макросы,unsafe-код и профессиональное тестирование — финальный уровень владения языком.",
        lessons: vec![macros(), unsafe_rust(), testing()],
    }
}

fn macros() -> Lesson {
    Lesson {
        id: "adv-macros",
        title: "Макросы",
        blocks: vec![
            Block::P("Макросы — код, который пишет код на этапе компиляции. Два вида: декларативные (macro_rules!, сопоставление с образцом) и процедурные (derive, attr, function-like — полноценные Rust-программы)."),
            Block::Code {
                caption: "Декларативный макрос",
                code: r##"// $x:expr — захватить выражение; $(...),* — повторение
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

macro_rules! assert_close {
    ($a:expr, $b:expr) => {
        assert!(
            ($a - $b).abs() < 1e-9,
            "{} != {} (слишком далеко)", $a, $b
        );
    };
}

fn main() {
    let m = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    println!("{m:?}");

    assert_close!(0.1 + 0.2, 0.3);
    println!("ок");
}"##,
                runnable: true,
            },
            Block::List(&[
                "Фрагмент-спецификаторы: expr, ident, ty, pat, literal, stmt, block, path, tt.",
                "$( ... ),* — ноль и больше повторений; + — хотя бы одно; $(,)? — опциональная запятая.",
                "Макросы расширяются до разбора типов — принимают переменное число аргументов любого вида.",
                "Экспорт: #[macro_export] над макросом делает его доступным на уровне крейта.",
            ]),
            Block::Heading("Процедурные макросы: derive"),
            Block::Code {
                caption: "Собственный #[derive]",
                code: r##"// Отдельный крейт с proc-macro = true:
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput};
//
// #[proc_macro_derive(Hello)]
// pub fn derive_hello(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as DeriveInput);
//     let name = &ast.ident;
//     quote! {
//         impl #name {
//             pub fn hello() -> &'static str {
//                 concat!("привет от ", stringify!(#name))
//             }
//         }
//     }.into()
// }

// Использование:
// #[derive(Hello)]
// struct Config;  // Config::hello() -> "привет от Config"
fn main() {
    println!("derive-макросы — так работают serde, Debug, Clone");
}"##,
                runnable: true,
            },
            Block::Note("Знакомые макросы — всё proc/declarative: println!, vec!, assert_eq!, #[derive(Debug)] (serde), tokio::main, async fn... Язык Rust сам во многом построен на макросах."),
            Block::Note("Правило гигиены: идентификаторы внутри macro_rules! не конфликтуют с вызывающим кодом — макросы гигиеничны."),
        ],
        quiz: vec![
            Question {
                text: "Чем декларативный макрос отличается от процедурного?",
                options: &[
                    "Декларативный — сопоставление с образцом в macro_rules!; процедурный — Rust-код, работающий с AST",
                    "Ничем",
                    "Процедурный — только в std",
                ],
                answer: 0,
                explain: "macro_rules! — шаблоны. Процедурные (derive/attr/fn-like) — программы над токенами; для них нужен отдельный крейт.",
            },
            Question {
                text: "Что означает $( ... ),* в macro_rules!?",
                options: &[
                    "Один необязательный аргумент",
                    "Повторение фрагмента ноль или более раз, разделённое запятыми",
                    "Комментарий",
                ],
                answer: 1,
                explain: "$(pattern),* — вариативная часть: так vec! принимает сколько угодно элементов.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите макрос my_vec! { a; b; c } (через ;), создающий Vec<i32> из элементов. Проверьте на трёх значениях.",
            starter: r##"macro_rules! my_vec {
    // TODO
}

fn main() {
    let v = my_vec![1; 2; 3];
    println!("{v:?}"); // [1, 2, 3]
}"##,
            solution: r##"macro_rules! my_vec {
    ($($item:expr);* $(;)?) => {{
        let mut v = Vec::new();
        $(v.push($item);)*
        v
    }};
}

fn main() {
    let v = my_vec![1; 2; 3];
    println!("{v:?}");
}"##,
        }),
    }
}

fn unsafe_rust() -> Lesson {
    Lesson {
        id: "adv-unsafe",
        title: "Unsafe Rust",
        blocks: vec![
            Block::P("unsafe — режим, где разрешены пять сверхспособностей: разыменование сырых указателей, вызов unsafe-функций, доступ/изменение мутабельной статической переменной, реализация unsafe-трейтов, доступ к union-полям. Всё остальное Rust проверяет и здесь."),
            Block::Code {
                caption: "Сырые указатели и unsafe-блок",
                code: r##"fn main() {
    let mut num = 5;

    // Сырые указатели: *const T и *mut T
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Разыменование — только в unsafe:
    unsafe {
        println!("r1 = {}", *r1);
        *r2 = 10;
        println!("num = {}", *r2);
    }
    println!("num = {num}");
}"##,
                runnable: true,
            },
            Block::List(&[
                "Сырые указатели могут быть нулевыми, висячими, нарушать алиасинг — ответственность на вас.",
                "unsafe НЕ отключает borrow checker: внутри unsafe-блока ссылки всё равно проверяются.",
                "Смысл: инкапсулировать unsafe в безопасной абстракции (как делает Vec внутри), наружу — безопасный API.",
            ]),
            Block::Code {
                caption: "Безопасная обёртка вокруг unsafe",
                code: r##"use std::slice;

// ОЖИДАНИЕ КОНТРАКТА: chunk не пуст, len корректен
fn split_at(values: &[u8], mid: usize) -> (&[u8], &[u8]) {
    let (a, b) = values.split_at(mid);
    (a, b)
}

fn main() {
    let data = [1u8, 2, 3, 4];

    // split_at_mut в std использует unsafe внутри —
    // но API безопасен: нарушить контракт извне невозможно
    let mut v = vec![1, 2, 3, 4];
    let (left, right) = v.split_at_mut(2);
    right[0] = 99;
    println!("{left:?} {right:?}");

    let _ = split_at(&data, 2);
}"##,
                runnable: true,
            },
            Block::Warn("Пять сверхспособностей unsafe не отменяют UB: разыменование висячего указателя в unsafe — неопределённое поведение. Правило: unsafe-код должен uphold инварианты, за которые отвечал бы компилятор. Пишите SAFETY-комментарии, гоняйте Miri."),
            Block::Note("Реальные применения: FFI (вызов C-библиотек), работа с ОС, быстрые структуры данных, встраиваемое железо. В прикладном коде вы почти не пишете unsafe — ecosystem делает это за вас."),
        ],
        quiz: vec![
            Question {
                text: "Что разрешает unsafe-блок?",
                options: &[
                    "Игнорировать все проверки компилятора",
                    "Пять операций: сырые указатели, unsafe fn, мутабельный static, unsafe-трейты, union-поля",
                    "Удалять данные из кучи вручную",
                ],
                answer: 1,
                explain: "unsafe открывает конкретный список супервозможностей. Borrow checker внутри unsafe продолжает работать.",
            },
            Question {
                text: "Правильный ли подход: библиотека использует unsafe внутри Vec::push, снаружи API безопасен?",
                options: &[
                    "Нет, unsafe всегда плохо",
                    "Да — инкапсулировать unsafe в проверенных абстракциях и есть философия Rust",
                    "Только в тестах",
                ],
                answer: 1,
                explain: "std сам так делает: unsafe в правильных руках внутри, безопасный интерфейс снаружи.",
            },
        ],
        exercise: Some(Exercise {
            task: "Используйте unsafe минимально: создайте сырой указатель на число, разыменуйте его в unsafe-блоке и напечатайте. Затем попробуйте НЕ в unsafe-блоке — убедитесь, что компилятор откажет.",
            starter: r##"fn main() {
    let x = 42;
    let ptr = &x as *const i32;
    // TODO: безопасное разыменование ptr
}"##,
            solution: r##"fn main() {
    let x = 42;
    let ptr = &x as *const i32;

    // SAFETY: указатель получен из живой ссылки x,
    // которая не изменяется во время разыменования
    let value = unsafe { *ptr };
    println!("{value}");

    // *ptr вне unsafe -> ошибка компиляции:
    // "operation is unsafe and requires unsafe function or block"
}"##,
        }),
    }
}

fn testing() -> Lesson {
    Lesson {
        id: "adv-test",
        title: "Тесты и документация",
        blocks: vec![
            Block::P("Тестирование встроено в язык: cargo test находит и запускает unit-тесты рядом с кодом, интеграционные в tests/ и примеры из документации. Отдельного фреймворка не нужно."),
            Block::Code {
                caption: "Unit-тесты в том же файле",
                code: r##"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

#[cfg(test)]          // компилируется только для cargo test
mod tests {
    use super::*;     // импорт тестируемого кода

    #[test]
    fn it_adds() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn division_by_zero_is_none() {
        assert!(divide(1.0, 0.0).is_none());
        assert_eq!(divide(9.0, 3.0), Some(3.0));
    }

    #[test]
    #[should_panic(expected = "вне границ")]
    fn panics_on_bad_index() {
        let v: Vec<i32> = vec![];
        v[0]; // паника
    }

    #[test]
    fn uses_result() -> Result<(), String> {
        let x = add(2, 2);
        if x == 4 { Ok(()) } else { Err(String::from("арифметика сломалась")) }
    }
}"##,
                runnable: false,
            },
            Block::List(&[
                "#[test] помечает тест; #[should_panic(expected = \"...\")] — тест паники.",
                "Тест может вернуть Result — удобно для оператора ?.",
                "assert!, assert_eq!, assert_ne! — базовые проверки; поля формата после строки — сообщения.",
                "cargo test имя_теста — фильтр; cargo test -- --nocapture — показать println.",
            ]),
            Block::Heading("Интеграционные тесты"),
            Block::Code {
                caption: "tests/api.rs — тестирует публичный API крейта",
                code: r##"// tests/api.rs — отдельный крейт, видит только pub-элементы
// use my_app::something_public;
//
// #[test]
// fn end_to_end() {
//     let input = "...";
//     let output = my_app::process(input);
//     assert_eq!(output, "ожидаемо");
// }"##,
                runnable: false,
            },
            Block::Heading("Doc-тесты — документация, которая не лжёт"),
            Block::Code {
                caption: "Примеры в /// проверяются cargo test",
                code: r##"/// Нормализует строку: трим + нижний регистр.
///
/// # Examples
///
/// ```
/// assert_eq!(normalize("  ПРИВЕТ "), "привет");
/// ```
pub fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

fn main() {
    println!("{}", normalize("  TEST "));
}"##,
                runnable: true,
            },
            Block::Note("Формула качества: unit-тесты — внутренности, интеграционные — контракты, doc-тесты — примеры для пользователей. cargo test прогоняет все три. clippy + fmt в CI — стандарт индустрии."),
            Block::Note("Бенчмарки: #[bench] устарел; criterion — стандарт для измерения производительности. Fuzzing: cargo-fuzz находит краши на случайных входах."),
        ],
        quiz: vec![
            Question {
                text: "Где находятся unit-тесты в Rust?",
                options: &[
                    "В отдельном каталоге unit/",
                    "Рядом с кодом, в модулях #[cfg(test)]",
                    "Только в tests/",
                ],
                answer: 1,
                explain: "Unit-тесты — в том же файле, в модулях #[cfg(test)] (видят приватные элементы). tests/ — интеграционные.",
            },
            Question {
                text: "Что делает cargo test с примерами из doc-комментариев?",
                options: &[
                    "Игнорирует",
                    "Компилирует и запускает их как тесты",
                    "Показывает предупреждение",
                ],
                answer: 1,
                explain: "Doc-тесты гарантируют: примеры в документации работают. Устаревший пример ломает сборку.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию is_palindrome(s: &str) -> bool (игнорируя регистр и небуквенные символы) и покройте её тремя unit-тестами: положительный, отрицательный и случай с пунктуацией («А роза упала на лапу Азора»).",
            starter: r##"fn is_palindrome(s: &str) -> bool {
    // TODO
    false
}

fn main() {
    println!("{}", is_palindrome("шалаш"));
}

#[cfg(test)]
mod tests {
    // TODO
}"##,
            solution: r##"fn is_palindrome(s: &str) -> bool {
    let clean: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    clean.iter().rev().eq(clean.iter())
}

fn main() {
    println!("{}", is_palindrome("шалаш"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_word() {
        assert!(is_palindrome("шалаш"));
        assert!(!is_palindrome("привет"));
    }

    #[test]
    fn ignores_case_and_punctuation() {
        assert!(is_palindrome("А роза упала на лапу Азора"));
    }

    #[test]
    fn empty_is_palindrome() {
        assert!(is_palindrome(""));
    }
}"##,
        }),
    }
}
