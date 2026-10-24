//! Модуль 3: Владение и заимствование.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🔑",
        title: "Владение и заимствование",
        blurb: "Главная особенность Rust: кто владеет данными, кто может их читать и изменять — решает компилятор.",
        lessons: vec![rules(), borrow(), slices()],
    }
}

fn rules() -> Lesson {
    Lesson {
        id: "own-rules",
        title: "Правила владения",
        blocks: vec![
            Block::P("В языках с GC памятью управляет сборщик мусора, в C/C++ — программист вручную. В Rust память управляется системой владения (ownership): компилятор проверяет её правила во время сборки, и никакой рантайм-накладной платы это не стоит."),
            Block::Heading("Три правила владения"),
            Block::List(&[
                "У каждого значения есть ровно один владелец (owner).",
                "Когда владелец выходит из области видимости, значение уничтожается (вызывается drop).",
                "Значение можно переместить (move) другому владельцу или одолжить (borrow) — но не одновременно мутировать и читать из нескольких мест.",
            ]),
            Block::Code {
                caption: "Область видимости и drop",
                code: r##"fn main() {
    {                      // область видимости начинается
        let s = String::from("привет");
        println!("{s}");
    }                      // s выходит из области — память освобождена

    // println!("{s}");    // ОШИБКА: s здесь не существует
}"##,
                runnable: false,
            },
            Block::Heading("Move: передача владения"),
            Block::Code {
                caption: "Присваивание перемещает значения с владением",
                code: r##"fn main() {
    let s1 = String::from("данные");
    let s2 = s1; // ВЛАДЕНИЕ ПЕРЕШЛО к s2

    // println!("{s1}"); // ОШИБКА: value borrowed after move
    println!("{s2}");    // ок
}"##,
                runnable: false,
            },
            Block::P("Почему нельзя просто скопировать? String хранит указатель на данные в куче. Если бы обе переменные владели одним буфером, при выходе из области каждая попыталась бы освободить его — двойное освобождение. Rust решает это радикально: после move старая переменная считается недействительной."),
            Block::Heading("Copy-типы — исключение"),
            Block::Code {
                caption: "Простые типы копируются",
                code: r##"fn main() {
    let x = 5;
    let y = x; // копирование, не перемещение
    println!("{x} {y}"); // обе живут — ок

    let point = (1, 2);
    let point2 = point; // кортеж из Copy-типов тоже Copy
    println!("{point:?} {point2:?}");
}"##,
                runnable: true,
            },
            Block::List(&[
                "Copy-типы: все целые и дробные числа, bool, char, а также кортежи и массивы, целиком состоящие из Copy-типов.",
                "Типы с владением (String, Vec, Box...) — не Copy: их присваивание — это move.",
                "Свой тип можно пометить #[derive(Clone, Copy)], если он состоит только из Copy-полей.",
            ]),
            Block::Heading("Функции и владение"),
            Block::Code {
                caption: "Передача в функцию тоже перемещает",
                code: r##"fn consume(s: String) {
    println!("забрал: {s}");
} // s здесь уничтожается

fn main() {
    let s = String::from("кошелёк");
    consume(s);
    // println!("{s}"); // ОШИБКА: владение ушло в consume и было потеряно

    let n = 42;
    give_number(n);
    println!("n всё ещё с нами: {n}"); // Copy-тип — ок
}

fn give_number(x: i32) {
    println!("{x}");
}"##,
                runnable: false,
            },
            Block::Note("Паттерн «отдать и вернуть»: функция может вернуть владение обратно — let s = take(s); Но постоянно гонять значения туда-обратно неудобно. Решение — ссылки и заимствование, следующий урок."),
        ],
        quiz: vec![
            Question {
                text: "Что происходит при let s2 = s1; если s1 — String?",
                options: &[
                    "Строка копируется, обе переменные действительны",
                    "Владение перемещается к s2, s1 больше нельзя использовать",
                    "s2 становится ссылкой на s1",
                ],
                answer: 1,
                explain: "String не Copy: присваивание перемещает владение. Использование s1 после move — ошибка компиляции (это защищает от двойного освобождения).",
            },
            Question {
                text: "Когда значение уничтожается?",
                options: &[
                    "Когда сборщик мусора решит",
                    "Когда владелец выходит из области видимости",
                    "В конце программы всегда",
                ],
                answer: 1,
                explain: "Rust вставляет вызов drop в конец области видимости владельца — детерминированное освобождение без GC.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию, которая принимает String, возвращает её длину и возвращает владение строкой обратно вызывающему. Проверьте, что строку можно использовать после вызова.",
            starter: r##"fn main() {
    let s = String::from("владение");
    // TODO: вызовите len_and_back так, чтобы s осталась жива
    println!("{s}");
}

// TODO: len_and_back — принимает String, печатает длину, возвращает String"##,
            solution: r##"fn main() {
    let s = String::from("владение");
    let s = len_and_back(s); // затенение: старый s уже невалиден
    println!("{s} жива");    // ок
}

fn len_and_back(s: String) -> String {
    println!("длина: {}", s.len());
    s // вернули владение
}"##,
        }),
    }
}

fn borrow() -> Lesson {
    Lesson {
        id: "own-borrow",
        title: "Ссылки и заимствование",
        blocks: vec![
            Block::P("Ссылка (&) позволяет использовать значение, не забирая владение. Это как аренда: владелец есть, а арендатор лишь пользуется. Дать ссылку — значит одолжить (borrow)."),
            Block::Code {
                caption: "Неизменяемая ссылка &T",
                code: r##"fn length(s: &String) -> usize {
    s.len()
} // ссылка умирает, владение строкой — нет

fn main() {
    let s = String::from("заимствование");
    let len = length(&s); // одалживаем
    println!("'{s}' имеет длину {len}"); // строка всё ещё наша!
}"##,
                runnable: true,
            },
            Block::Heading("Изменяемая ссылка &mut T"),
            Block::Code {
                caption: "Чтобы менять — нужно &mut",
                code: r##"fn add_suffix(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut s = String::from("готово");
    add_suffix(&mut s);
    println!("{s}"); // готово!
}"##,
                runnable: false,
            },
            Block::Heading("Два правила заимствования"),
            Block::List(&[
                "В любой момент может существовать сколько угодно неизменяемых ссылок (&T)...",
                "...либо РОВНО одна изменяемая ссылка (&mut T) — и тогда никаких других ссылок.",
                "Ссылка не может жить дольше владельца (висячих ссылок не бывает).",
            ]),
            Block::Code {
                caption: "Почему это правило — гениально",
                code: r##"fn main() {
    let mut data = vec![1, 2, 3];

    let first = &data[0];        // неизменяемая ссылка
    // data.push(4);             // ОШИБКА: нельзя мутировать, пока есть читатель
    println!("{first}");         // 1

    // После последнего использования first — можно мутировать:
    data.push(4);                // ок
    println!("{data:?}");
}"##,
                runnable: false,
            },
            Block::P("push может перевыделить память буфера, и first указал бы в никуда. Rust запрещает такую ситуацию на этапе компиляции — это классическая проблема итерации с мутацией, которую в других языках ловят только в рантайме."),
            Block::Code {
                caption: "Две &mut — нельзя",
                code: r##"fn main() {
    let mut s = String::from("x");
    let r1 = &mut s;
    // let r2 = &mut s;      // ОШИБКА: вторая изменяемая ссылка
    // println!("{s} {r1}"); // ОШИБКА: нельзя читать, пока одолжено для мутации
    r1.push('!');
    println!("{s}");
}"##,
                runnable: false,
            },
            Block::Note("Правила работают в пределах жизни ссылок в коде. Если ссылка использована до создания другой — компилятор уже не считает конфликт (Non-Lexical Lifetimes)."),
            Block::Heading("Тени ссылок и dereference"),
            Block::Code {
                caption: "Разыменование и авто-разыменование",
                code: r##"fn shout(s: &str) {
    // &String автоматически превращается в &str (deref coercion)
    println!("{}!", s.to_uppercase());
}

fn main() {
    let s = String::from("тихо");
    shout(&s);              // передаём &String, функция ждёт &str
    let r = &s;
    println!("{} и {}", r.len(), s.len()); // *r разыменовывает неявно
}"##,
                runnable: true,
            },
            Block::Warn("Метод len() у &String вызывает len() самой строки — Rust автоматически разыменовывает при вызове методов (auto-deref). Не путайте: *r — явное разыменование."),
        ],
        quiz: vec![
            Question {
                text: "Сколько изменяемых ссылок на одно значение может существовать одновременно?",
                options: &["Сколько угодно", "Не более одной", "Ровно две"],
                answer: 1,
                explain: "Правило: много &T ИЛИ одна &mut T. Это исключает гонки данных и паттерн 'iterator invalidation' на этапе компиляции.",
            },
            Question {
                text: "Что произойдёт: let mut v = vec![1]; let r = &v[0]; v.push(2); println!(\"{r}\"); ?",
                options: &[
                    "Напечатает 1",
                    "Ошибка компиляции: нельзя мутировать v, пока активна ссылка r",
                    "Паника в рантайме",
                ],
                answer: 1,
                explain: "push может перевыделить буфер, и r стал бы висячим. Rust ловит это при компиляции — одна из главных причин, почему Rust безопасен.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию longest_word(text: &str) -> &str, возвращающую самое длинное слово в строке (разделитель — пробел). Подсказка: split_whitespace(), max_by_key().",
            starter: r##"fn main() {
    let text = "rust — язык системного программирования";
    println!("{}", longest_word(text));
}

fn longest_word(text: &str) -> &str {
    // TODO
    ""
}"##,
            solution: r##"fn main() {
    let text = "rust — язык системного программирования";
    println!("{}", longest_word(text)); // программирования
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|w| w.chars().count())
        .unwrap_or("")
}"##,
        }),
    }
}

fn slices() -> Lesson {
    Lesson {
        id: "own-slices",
        title: "Срезы",
        blocks: vec![
            Block::P("Срез (slice) — ссылка на непрерывную часть коллекции: &[T] для массивов и векторов, &str — для строк. Срез не владеет данными, это «окно» на чужие байты."),
            Block::Code {
                caption: "Строковые срезы",
                code: r##"fn main() {
    let s = String::from("Привет, мир");

    let hello = &s[0..6]; // или &s[..6]
    let world = &s[8..];  // до конца
    let all = &s[..];     // вся строка

    println!("{hello} {world} {all}");
}"##,
                runnable: true,
            },
            Block::Warn("Срезы строк режутся по байтам, а не по символам! Кириллица занимает 2 байта в UTF-8. Если граница попадёт в середину символа — паника: byte index N is not a char boundary."),
            Block::Code {
                caption: "Срезы векторов",
                code: r##"fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum() // &[i32] работает и с Vec, и с массивом
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", sum(&v));      // весь вектор как срез
    println!("{}", sum(&v[1..4])); // 2+3+4 = 9

    let arr = [10, 20];
    println!("{}", sum(&arr)); // массив тоже приводится к срезу
}"##,
                runnable: true,
            },
            Block::Note("Функция, принимающая &[i32], — самый гибкий вариант: подойдёт и Vec, и массив, и часть вектора. Это идиома «принимай срез» (pass slices). Аналогично принимайте &str вместо &String."),
            Block::Code {
                caption: "Идиома: &str в сигнатурах",
                code: r##"fn greet(name: &str) {
    println!("Привет, {name}!");
}

fn main() {
    let owned = String::from("Мир");
    greet(&owned);  // String -> &str бесплатно
    greet("мир");   // строковый литерал тоже &str
}"##,
                runnable: true,
            },
        ],
        quiz: vec![
            Question {
                text: "Почему в функциях лучше принимать &str, а не &String?",
                options: &[
                    "&str быстрее компилируется",
                    "&str принимает и String, и литералы, и срезы — максимум гибкости для вызывающего",
                    "String устарел",
                ],
                answer: 1,
                explain: "Это правило обобщённости: &[T] и &str — широкие интерфейсы. &String — узкий, бесполезно ограничивающий.",
            },
            Question {
                text: "Что вернёт \"привет\"[..2] на UTF-8?",
                options: &["\"пр\"", "Панику — граница внутри символа не гарантирована", "\"п\""],
                answer: 1,
                explain: "Кириллическая 'п' занимает 2 байта, значит граница ..2 как раз попадает на конец первого символа и НЕ паникует. Но \"привет\"[..1] паникует: 1 байт — середина символа. Границы байтов — рискованная тема с UTF-8.",
            },
        ],
        exercise: Some(Exercise {
            task: "Напишите функцию first_word(s: &str) -> &str, возвращающую первое слово до пробела (без метода split — вручную через find или цикл по байтам char_indices).",
            starter: r##"fn main() {
    println!("{}", first_word("hello world"));  // hello
    println!("{}", first_word("толькослово"));  // толькослово
    println!("[{}]", first_word(""));           // пустая строка
}

fn first_word(s: &str) -> &str {
    // TODO: найти индекс первого пробела и взять срез
    s
}"##,
            solution: r##"fn main() {
    println!("{}", first_word("hello world"));
    println!("{}", first_word("толькослово"));
    println!("[{}]", first_word(""));
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s, // пробела нет — вся строка и есть слово
    }
}"##,
        }),
    }
}
