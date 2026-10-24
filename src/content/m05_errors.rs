//! Модуль 5: Обработка ошибок.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🚨",
        title: "Обработка ошибок",
        blurb: "Паники для багов, Result для ожидаемых сбоев, оператор ? для элегантных цепочек.",
        lessons: vec![panic_unwrap(), result()],
    }
}

fn panic_unwrap() -> Lesson {
    Lesson {
        id: "err-panic",
        title: "panic! и методы-unwrap'ы",
        blocks: vec![
            Block::P("В Rust два вида сбоев. Паника (panic!) — неисправимая ситуация: нарушение инварианта, выход за границы массива, баг в логике. Ожидаемая ошибка (Result) — нормальный сценарий: файл не найден, сеть отвалилась, пользователь ввёл мусор."),
            Block::Heading("Паника"),
            Block::Code {
                caption: "panic! останавливает поток",
                code: r##"fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[10]); // паника: index out of bounds
}"##,
                runnable: false,
            },
            Block::Output("thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'"),
            Block::List(&[
                "По умолчанию паника раскручивает стек (unwinding) и печатает backtrace.",
                "RUST_BACKTRACE=1 cargo run — покажет стек вызовов.",
                "panic = \"abort\" в Cargo.toml — немедленное прерывание процесса (меньше бинарник).",
            ]),
            Block::Heading("Когда паника уместна"),
            Block::List(&[
                "Баг: инвариант нарушен, продолжать опасно и бессмысленно.",
                "Примеры, тесты, прототипы — где обработка ошибки только мешает.",
                "Никогда — для ожидаемых ситуаций (пользователь опечатался в пути файла).",
            ]),
            Block::Heading("Семейство unwrap"),
            Block::Code {
                caption: "Быстрые (опасные) способы достать значение",
                code: r##"fn main() {
    let maybe: Option<i32> = Some(42);
    println!("{}", maybe.unwrap());   // паника, если None

    let result: Result<i32, String> = Err("нет сети".to_string());
    // result.unwrap();  // паника: called Result::unwrap() on an Err

    // expect — то же, но с вашим сообщением (лучше в проде)
    // result.expect("база данных недоступна");

    // Безопасные альтернативы:
    println!("{}", maybe.unwrap_or(0));             // значение или 0
    println!("{:?}", result.ok());                  // Result -> Option
    let handled = result.unwrap_or_else(|e| {
        println!("логируем: {e}");
        -1
    });
    println!("{handled}");
}"##,
                runnable: true,
            },
            Block::Warn("unwrap() в прикладном коде — красный флаг: вы обещаете компилятору, что ошибки не будет, а он вам не верит. В библиотеках unwrap запрещён почти всегда; в примерах и тестах — нормально."),
        ],
        quiz: vec![
            Question {
                text: "Когда паника — правильный выбор?",
                options: &[
                    "Когда пользователь ввёл неверный путь к файлу",
                    "Когда нарушен внутренний инвариант и продолжать бессмысленно",
                    "Когда сеть медленная",
                ],
                answer: 1,
                explain: "Паника — для неисправимых багов. Ожидаемые сбои (файл, сеть, ввод) — это Result.",
            },
            Question {
                text: "Чем expect лучше unwrap?",
                options: &[
                    "Ничем, синонимы",
                    "Позволяет указать сообщение о причине паники — легче диагностировать",
                    "Не паникует",
                ],
                answer: 1,
                explain: "expect(\"контекст\") паникует с вашим сообщением: 'контекст: причина'. Так находятся причины быстрее.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию divide(a: f64, b: f64), которая паникует с понятным сообщением при делении на ноль и возвращает частное иначе. Проверьте оба случая.",
            starter: r##"fn divide(a: f64, b: f64) -> f64 {
    // TODO
    a / b
}

fn main() {
    println!("{}", divide(10.0, 4.0));
    // println!("{}", divide(1.0, 0.0)); // раскомментируйте после реализации
}"##,
            solution: r##"fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("деление на ноль: divide({a}, {b})");
    }
    a / b
}

fn main() {
    println!("{}", divide(10.0, 4.0)); // 2.5
    // divide(1.0, 0.0) -> паника с вашим сообщением
}"##,
        }),
    }
}

fn result() -> Lesson {
    Lesson {
        id: "err-result",
        title: "Result и оператор ?",
        blocks: vec![
            Block::P("Result<T, E> — enum с двумя вариантами: Ok(T) и Err(E). Это тип «может быть успех или ошибка», и компилятор заставляет проверить его перед использованием. Ошибки становятся частью сигнатуры функции."),
            Block::Code {
                caption: "Result вручную",
                code: r##"use std::num::ParseIntError;

fn parse_age(input: &str) -> Result<u8, ParseIntError> {
    input.parse::<u8>() // parse сам возвращает Result
}

fn main() {
    match parse_age("27") {
        Ok(age) => println!("возраст: {age}"),
        Err(e) => println!("ошибка разбора: {e}"),
    }

    match parse_age("двести") {
        Ok(age) => println!("возраст: {age}"),
        Err(e) => println!("ошибка разбора: {e}"),
    }
}"##,
                runnable: true,
            },
            Block::Heading("Оператор ? — распространение ошибки"),
            Block::Code {
                caption: "? возвращает Err наверх автоматически",
                code: r##"use std::num::ParseIntError;

fn sum_pair(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;  // Err -> сразу return Err(...)
    let y = b.parse::<i32>()?;  // Ok -> распакованное значение
    Ok(x + y)
}

fn main() {
    println!("{:?}", sum_pair("3", "4"));     // Ok(7)
    println!("{:?}", sum_pair("3", "xy"));    // Err(ParseIntError { .. })
}"##,
                runnable: true,
            },
            Block::P("Оператор `?` — сахар для match: если Result — Ok, достаём значение и продолжаем; если Err — немедленно возвращаем ошибку из функции. Требование: тип ошибки функции должен позволять преобразование (From) из типа ошибки выражения."),
            Block::Heading("main тоже может возвращать Result"),
            Block::Code {
                caption: "Result в main",
                code: r##"use std::fs;

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("config.toml")?;
    println!("{} байт", content.len());
    Ok(())
}"##,
                runnable: false,
            },
            Block::Heading("Свои типы ошибок и цепочки"),
            Block::Code {
                caption: "Комбинаторы Result",
                code: r##"fn double(x: i32) -> Result<i32, String> {
    if x < 0 { Err(String::from("отрицательное")) } else { Ok(x * 2) }
}

fn main() {
    // map — преобразовать Ok
    println!("{:?}", double(3).map(|v| v + 1)); // Ok(7)

    // and_then — плоская цепочка вычислений с ошибками
    let chain = double(3).and_then(|v| double(v)).and_then(|v| double(v));
    println!("{:?}", chain); // Ok(24)

    let broken = double(-1).and_then(|v| double(v));
    println!("{:?}", broken); // Err("отрицательное")

    // Ок-путь и Err-путь в одном стиле
    let fallback = double(-1).unwrap_or(0);
    println!("{fallback}");
}"##,
                runnable: true,
            },
            Block::Note("Для реальных проектов с несколькими источниками ошибок используют крейт thiserror (для библиотек: автоматический вывод Display/Error) и anyhow (для приложений: динамическая ошибка с контекстом). Мы познакомимся с ними в проекте."),
            Block::Warn("Проверяйте предупреждение unused_must_use: Result игнорировать нельзя — компилятор напомнит, что ошибку надо обработать (или явно отбросить: let _ = ...)."),
        ],
        quiz: vec![
            Question {
                text: "Что делает оператор ? применительно к Result?",
                options: &[
                    "Печатает ошибку и продолжает",
                    "Возвращает Err из функции немедленно или распаковывает Ok",
                    "Превращает Err в панику",
                ],
                answer: 1,
                explain: "? — сахар для match: Ok(x) -> продолжаем с x, Err(e) -> return Err(e.into()).",
            },
            Question {
                text: "Почему ошибки в Rust — это значения (Result), а не исключения?",
                options: &[
                    "Так меньше бинарник",
                    "Ошибки видны в сигнатурах функций, обязательны к обработке и не перескакивают уровни неожиданно",
                    "Так исторически сложилось",
                ],
                answer: 1,
                explain: "Result делает путь ошибок явным: компилятор требует обработки, а стек вызовов не взрывается невидимыми исключениями.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите цепочку: parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> — распарсите строку в i32 и удвойте. Используйте оператор ?. Проверьте на \"21\" и \"abc\".",
            starter: r##"fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    // TODO
    Ok(0)
}

fn main() {
    println!("{:?}", parse_and_double("21"));
    println!("{:?}", parse_and_double("abc"));
}"##,
            solution: r##"fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?}", parse_and_double("21")); // Ok(42)
    println!("{:?}", parse_and_double("abc")); // Err(...)
}"##,
        }),
    }
}
