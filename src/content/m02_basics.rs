//! Модуль 2: Основы языка.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🔤",
        title: "Основы языка",
        blurb: "Переменные, типы, функции и управление потоком — фундамент, на котором стоит всё остальное.",
        lessons: vec![vars(), types(), functions(), flow()],
    }
}

fn vars() -> Lesson {
    Lesson {
        id: "basics-vars",
        title: "Переменные и изменяемость",
        blocks: vec![
            Block::P("Ключевая идея Rust: по умолчанию всё неизменяемо (immutable). Переменная, объявленная через `let`, не может менять значение. Чтобы разрешить изменение, нужно явно попросить: `let mut`."),
            Block::Code {
                caption: "Неизменяемость по умолчанию",
                code: r##"fn main() {
    let x = 5;
    println!("x = {x}");
    x = 6; // ОШИБКА: cannot assign twice to immutable variable
}"##,
                runnable: false,
            },
            Block::Code {
                caption: "mut — явное разрешение на изменение",
                code: r##"fn main() {
    let mut count = 0;
    count += 1;
    count += 1;
    println!("count = {count}"); // count = 2
}"##,
                runnable: true,
            },
            Block::P("Зачем неизменяемость по умолчанию? Это часть философии безопасности: читая код, вы всегда видите, что может изменяться, а что — нет. Компилятор предупреждёт, если `mut` не нужен, и не даст изменить то, что менять нельзя."),
            Block::Heading("Затенение (shadowing)"),
            Block::P("Можно объявить новую переменную с тем же именем — прежняя «затеняется». Это не мутация: это новое связывание, у которого может быть даже другой тип."),
            Block::Code {
                caption: "Затенение позволяет переиспользовать имя",
                code: r##"fn main() {
    let spaces = "   ";        // &str — строковый срез
    let spaces = spaces.len(); // usize — другое ТИП!

    println!("Пробелов: {spaces}"); // Пробелов: 3
}"##,
                runnable: true,
            },
            Block::Warn("Затенение работает только внутри той же области видимости. `let mut x = 5; x = 6;` — мутация (тип менять нельзя). `let x = 5; let x = 6;` — затенение (тип может быть другим)."),
            Block::Heading("Константы"),
            Block::Code {
                caption: "const — вычисляется на этапе компиляции",
                code: r##"const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Максимум: {MAX_POINTS}");
}"##,
                runnable: false,
            },
            Block::List(&[
                "Константы объявляются словом `const`, тип обязателен, имя — ВЕРХНИЙ_РЕГИСТР_С_ПОДЧЁРКИВАНИЯМИ.",
                "Константа может жить в любой области, в том числе глобальной, и живёт всю программу.",
                "Подчёркивания в числах — для читаемости: 1_000_000.",
            ]),
        ],
        quiz: vec![
            Question {
                text: "Чем затенение (shadowing) отличается от мутации?",
                options: &[
                    "Ничем, это синонимы",
                    "Затенение создаёт новое связывание, даже с другим типом; мутация меняет значение той же переменной",
                    "Затенение работает только с константами",
                ],
                answer: 1,
                explain: "`let x = ...` создаёт новую переменную (тип может отличаться). Мутация `x = ...` изменяет существующую переменную — тип изменить нельзя.",
            },
            Question {
                text: "Что произойдёт с этим кодом: let a = 1; let a = a + 1; println!(\"{a}\");?",
                options: &["Ошибка компиляции", "Напечатает 2", "Напечатает 1"],
                answer: 1,
                explain: "Вторая let затеняет первую: a = 1 + 1 = 2. Это корректный и распространённый приём.",
            },
        ],
        exercise: Some(Exercise {
            task: "Объявите неизменяемую переменную-счётчик, затем используйте затенение, чтобы превратить её в строку «счётчик: N». Напечатайте результат.",
            starter: r##"fn main() {
    let count = 42;
    // TODO: затените count строкой и напечатайте "счётчик: 42"
    println!("{count}");
}"##,
            solution: r##"fn main() {
    let count = 42;
    let count = format!("счётчик: {count}");
    println!("{count}"); // счётчик: 42
}"##,
        }),
    }
}

fn types() -> Lesson {
    Lesson {
        id: "basics-types",
        title: "Типы данных",
        blocks: vec![
            Block::P("Rust — статически типизированный язык: тип каждой переменной известен при компиляции. Но тип часто можно не писать — компилятор выведет его сам (type inference)."),
            Block::Heading("Скалярные типы"),
            Block::List(&[
                "Целые: i8, i16, i32, i64, i128, isize и беззнаковые u8..u128, usize. По умолчанию — i32.",
                "Числа с плавающей точкой: f32 и f64 (по умолчанию f64).",
                "Логический: bool (true / false).",
                "Символ: char — один Unicode-символ в одинарных кавычках: 'ж', '🦀'.",
            ]),
            Block::Code {
                caption: "Вывод типов и явные аннотации",
                code: r##"fn main() {
    let a = 10;          // i32 — выводится по умолчанию
    let b: u64 = 10;     // тип указан явно
    let c = 2.5;         // f64
    let flag = true;     // bool
    let letter = 'Ж';    // char (одинарные кавычки!)
    let big = 1_000_000u64; // суффикс указывает тип

    println!("{a} {b} {c} {flag} {letter} {big}");
}"##,
                runnable: true,
            },
            Block::Note("usize — это размер указателя платформы (64 бита на 64-битных системах). Именно его используют индексы и размер коллекций: vec.len() возвращает usize."),
            Block::Heading("Переполнение"),
            Block::Code {
                caption: "Что бывает при переполнении",
                code: r##"fn main() {
    let max: u8 = 255;
    // в debug-сборке паника: attempt to add with overflow
    // let boom = max + 1;

    // в release: заворачивается по кругу
    let wrapped = max.wrapping_add(1);
    println!("{wrapped}"); // 0

    let checked = max.checked_add(1);
    println!("{:?}", checked); // None — переполнение обнаружено
}"##,
                runnable: true,
            },
            Block::Warn("В debug-сборке переполнение — паника, в release — заворачивание. Не полагайтесь ни на то, ни на другое: используйте checked_add / saturating_add / wrapping_add, когда переполнение возможно."),
            Block::Heading("Составные типы"),
            Block::Code {
                caption: "Кортеж и массив",
                code: r##"fn minmax(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

fn main() {
    // Кортеж: элементы разных типов, доступ по индексу
    let person: (&str, u8, char) = ("Ирина", 30, 'Ж');
    println!("{} {} {}", person.0, person.1, person.2);
    let (name, age, _) = person; // деструктуризация, _ игнорирует элемент
    println!("{name}, {age} лет");

    // Массив: элементы одного типа, размер известен на этапе компиляции
    let days: [&str; 3] = ["пн", "вт", "ср"];
    println!("Второй день: {}", days[1]);

    let (lo, hi) = minmax(5, 2);
    println!("min={lo} max={hi}");
}"##,
                runnable: true,
            },
            Block::Note("Массив `[T; N]` живёт в стеке и имеет фиксированный размер. Для динамических коллекций используется вектор `Vec<T>` — о нём позже."),
        ],
        quiz: vec![
            Question {
                text: "Какой тип по умолчанию у целочисленного литерала 10?",
                options: &["i64", "i32", "usize"],
                answer: 1,
                explain: "Без явного указания целые числа выводятся как i32 — быстрый размер даже на 64-битных системах.",
            },
            Question {
                text: "Чем кортеж отличается от массива?",
                options: &[
                    "Ничем",
                    "Кортеж может содержать элементы разных типов; массив — элементы одного типа фиксированной длины",
                    "Массив может менять размер в рантайме",
                ],
                answer: 1,
                explain: "Кортеж (T1, T2, ...) гетерогенный, массив [T; N] гомогенный и фиксированной длины. Динамическая коллекция — Vec<T>.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию swap(a: i32, b: i32) -> (i32, i32), возвращающую значения в обратном порядке. Используйте её и деструктуризацию кортежа.",
            starter: r##"fn main() {
    let a = 3;
    let b = 7;
    // TODO: получите (b, a) через swap
    println!("a={a} b={b}");
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    // TODO
    (a, b)
}"##,
            solution: r##"fn main() {
    let a = 3;
    let b = 7;
    let (a, b) = swap(a, b);
    println!("a={a} b={b}"); // a=7 b=3
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}"##,
        }),
    }
}

fn functions() -> Lesson {
    Lesson {
        id: "basics-fn",
        title: "Функции и выражения",
        blocks: vec![
            Block::P("В Rust почти всё — выражение: `if`, блок `{}`, `match` возвращают значение. Функции при этом — способ организации кода, а операторы `;` превращают выражения в операторы."),
            Block::Heading("Выражения vs операторы"),
            Block::Code {
                caption: "Блок — это выражение",
                code: r##"fn main() {
    let y = {
        let x = 3;
        x + 1 // без точки с запятой — значение блока
    };
    println!("y = {y}"); // y = 4
}"##,
                runnable: true,
            },
            Block::P("Если поставить `;` после `x + 1`, блок вернёт `()` — пустой тип unit, и y станет им. Это одна из самых частых ошибок новичков."),
            Block::Heading("Ранний возврат"),
            Block::Code {
                caption: "return для раннего выхода",
                code: r##"fn abs_diff(a: i32, b: i32) -> i32 {
    if a == b {
        return 0;
    }
    // дальше — только когда a != b
    if a > b { a - b } else { b - a }
}

fn main() {
    println!("{}", abs_diff(3, 9));  // 6
    println!("{}", abs_diff(9, 3));  // 6
    println!("{}", abs_diff(5, 5));  // 0
}"##,
                runnable: true,
            },
            Block::Heading("Документируем и тестируем"),
            Block::Code {
                caption: "Doc-комментарий с примером",
                code: r##"/// Возводит число в квадрат.
///
/// # Примеры
/// ```
/// assert_eq!(square(3), 9);
/// ```
fn square(x: i64) -> i64 {
    x * x
}

fn main() {
    println!("{}", square(7));
}"##,
                runnable: true,
            },
            Block::Note("Примеры в doc-комментариях — настоящие тесты: cargo test запускает их. Документация Rust всегда актуальна, потому что неактуальные примеры ломают сборку тестов."),
        ],
        quiz: vec![
            Question {
                text: "Что вернёт блок let z = { 5; }; ?",
                options: &["5", "Ошибка компиляции", "unit — пустой кортеж ()"],
                answer: 2,
                explain: "Точка с запятой отбрасывает значение, блок становится оператором и возвращает unit. y получит тип ().",
            },
            Question {
                text: "Зачем нужен явный return, если последнее выражение возвращается само?",
                options: &[
                    "Ни для чего, он запрещён",
                    "Для раннего выхода из функции на определённой ветке",
                    "Только для рекурсии",
                ],
                answer: 1,
                explain: "Идиоматично возвращать последнее выражение без return, но для раннего выхода (guard clauses) return — норма.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию fizzbuzz(n: u32), которая печатает для чисел 1..=n: «Fizz» для кратных 3, «Buzz» — кратных 5, «FizzBuzz» — кратных 15, иначе само число.",
            starter: r##"fn main() {
    fizzbuzz(15);
}

fn fizzbuzz(n: u32) {
    // TODO: цикл от 1 до n включительно
}"##,
            solution: r##"fn main() {
    fizzbuzz(15);
}

fn fizzbuzz(n: u32) {
    for i in 1..=n {
        let out = if i % 15 == 0 {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        };
        println!("{out}");
    }
}"##,
        }),
    }
}

fn flow() -> Lesson {
    Lesson {
        id: "basics-flow",
        title: "Управление потоком",
        blocks: vec![
            Block::Heading("if — выражение"),
            Block::Code {
                caption: "if возвращает значение",
                code: r##"fn main() {
    let n = 7;
    let parity = if n % 2 == 0 { "чётное" } else { "нечётное" };
    println!("{n} — {parity}");

    // Ветви должны иметь один тип!
    // let bad = if n > 0 { 1 } else { "минус" }; // ОШИБКА
}"##,
                runnable: true,
            },
            Block::Heading("Циклы"),
            Block::Code {
                caption: "loop, while, for и range",
                code: r##"fn main() {
    // loop — бесконечный цикл, выход через break (может вернуть значение)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop вернул {result}"); // 20

    // while — цикл с условием
    let mut n = 3;
    while n > 0 {
        print!("{n}..");
        n -= 1;
    }
    println!("поехали!");

    // for — перебор диапазонов и коллекций (самый частый)
    for i in 0..5 {        // 0,1,2,3,4 — правая граница исключена
        print!("{i} ");
    }
    println!();
    for i in (0..=5).rev() { // включительно, в обратном порядке
        print!("{i} ");
    }
    println!();
}"##,
                runnable: true,
            },
            Block::Output("loop вернул 20\n3..2..1..поехали!\n0 1 2 3 4 \n5 4 3 2 1 0 "),
            Block::Note("break и continue работают во всех циклах. Метки (label) позволяют выйти из вложенных циклов: 'outer: loop { ... break 'outer; }"),
            Block::Heading("match — первое знакомство"),
            Block::Code {
                caption: "match исчерпывающе проверяет все варианты",
                code: r##"fn main() {
    let coin = 2;
    let msg = match coin {
        1 => "один цент",
        5 => "пять центов",
        10 | 25 => "десять или двадцать пять", // несколько значений
        1..=99 => "другая мелочь",             // диапазон
        _ => "не мелочь",                      // _ — всё остальное
    };
    println!("{msg}"); // другая мелочь
}"##,
                runnable: true,
            },
            Block::Warn("match обязан покрывать ВСЕ возможные значения — компилятор проверяет исчерпывающе. Ветка _ (wildcard) нужна, когда явно перечислены не все варианты. Это основа безопасной обработки ошибок, о ней поговорим отдельно."),
        ],
        quiz: vec![
            Question {
                text: "Сколько раз выполнится тело цикла: for i in 0..3 { } ?",
                options: &["2", "3", "4"],
                answer: 1,
                explain: "Диапазон 0..3 включает 0, 1, 2 — три итерации. Правая граница исключается; 0..=3 — четыре.",
            },
            Question {
                text: "Что вернёт выражение let x = loop { break 7; }; ?",
                options: &["Ошибка: break не возвращает значений", "7", "unit ()"],
                answer: 1,
                explain: "break с выражением прерывает цикл и возвращает это выражение как значение loop — уникальная особенность Rust.",
            },
        ],
        exercise: Some(Exercise {
            task: "Используя match по числу, напишите функцию grade(score: u32) -> &'static str: 90..=100 — «отлично», 75..=89 — «хорошо», 60..=74 — «удовлетворительно», 0..=59 — «неудовлетворительно», больше 100 — «ошибка».",
            starter: r##"fn main() {
    for s in [55, 62, 88, 95, 120] {
        println!("{s} -> {}", grade(s));
    }
}

fn grade(score: u32) -> &'static str {
    // TODO: match с диапазонами
    "?"
}"##,
            solution: r##"fn main() {
    for s in [55, 62, 88, 95, 120] {
        println!("{s} -> {}", grade(s));
    }
}

fn grade(score: u32) -> &'static str {
    match score {
        90..=100 => "отлично",
        75..=89 => "хорошо",
        60..=74 => "удовлетворительно",
        0..=59 => "неудовлетворительно",
        _ => "ошибка",
    }
}"##,
        }),
    }
}
