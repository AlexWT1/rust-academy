//! Модуль 9: Конкурентность.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🧵",
        title: "Конкурентность",
        blurb: "Потоки и каналы, безопасный обмен через Arc/Mutex и современный async/await.",
        lessons: vec![threads(), shared(), async_()],
    }
}

fn threads() -> Lesson {
    Lesson {
        id: "conc-threads",
        title: "Потоки и каналы",
        blocks: vec![
            Block::P("std::thread создаёт настоящие потоки ОС. Замыкание, переданное в поток, должно быть move: поток может пережить область, где созданы данные, поэтому владение обязательно переносится."),
            Block::Code {
                caption: "Потоки и join",
                code: r##"use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // data принадлежит потоку
        println!("поток: сумма {:?}", data.iter().sum::<i32>());
        42 // возвращаемое значение
    });

    println!("main работает параллельно");

    let result = handle.join().expect("поток паниковал");
    println!("поток вернул {result}");
}"##,
                runnable: true,
            },
            Block::Note("join() блокирует текущий поток до завершения дочернего и возвращает его результат. Без join поток-потомок может быть отрезан при выходе из main."),
            Block::Heading("Каналы: обмен сообщениями"),
            Block::P("«Не общайтесь через общую память — общайтесь, разделяя память через сообщения». mpsc (multiple producer, single consumer) — канал из std."),
            Block::Code {
                caption: "mpsc-канал",
                code: r##"use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Клонируем отправителя для нескольких производителей
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("сообщение от воркера {id}")).unwrap();
        });
    }
    drop(tx); // перестаём сами — иначе rx не увидит конца

    for msg in rx { // итерация закончится, когда все tx закрыты
        println!("получено: {msg}");
    }
}"##,
                runnable: true,
            },
            Block::List(&[
                "send(value) — перемещает значение в канал (владение передаётся!).",
                "recv() — блокирует до сообщения; try_recv() — мгновенно Option.",
                "Канал закрывается, когда все отправители drop — итерация по rx завершается.",
            ]),
            Block::Warn("send требует move: значение уходит одному получателю. Так Rust исключает гонки: данные не могут одновременно «лететь» в канал и читаться отправителем."),
        ],
        quiz: vec![
            Question {
                text: "Почему thread::spawn требует move?",
                options: &[
                    "Для стиля",
                    "Поток может жить дольше создавшей функции — нужно забрать владение данными",
                    "move ускоряет запуск",
                ],
                answer: 1,
                explain: "Без move замыкание ссылалось бы на переменные, которые могут умереть раньше потока. Владение решает это statically.",
            },
            Question {
                text: "Когда итерация по rx (получателю канала) завершается?",
                options: &["Никогда", "Когда все отправители drop", "После первого сообщения"],
                answer: 1,
                explain: "mpsc: канал закрыт, когда закрыты все tx. for по rx ждёт сообщений и завершается на закрытии.",
            },
        ],
        exercise: Some(Exercise {
            task: "Запустите два потока: первый вычисляет сумму 1..=1_000_000, второй — произведение первых 10 чисел. Соберите оба результата через join и напечатайте.",
            starter: r##"use std::thread;

fn main() {
    // TODO: два spawn + join
}"##,
            solution: r##"use std::thread;

fn main() {
    let h1 = thread::spawn(|| (1..=1_000_000u64).sum::<u64>());
    let h2 = thread::spawn(|| (1..=10u64).product::<u64>());

    let (sum, prod) = (h1.join().unwrap(), h2.join().unwrap());
    println!("сумма: {sum}, произведение: {prod}");
}"##,
        }),
    }
}

fn shared() -> Lesson {
    Lesson {
        id: "conc-shared",
        title: "Arc и Mutex",
        blocks: vec![
            Block::P("Общая изменяемая память между потоками — Arc<Mutex<T>>. Arc — атомарный счётчик ссылок (многопоточный Rc), Mutex — взаимное исключение: доступ к данным только у одного потока, остальные ждут."),
            Block::Code {
                caption: "Счётчик из нескольких потоков",
                code: r##"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // lock() возвращает Result<MutexGuard>; guard
            // освобождает мьютекс при выходе из области
            let mut num = counter.lock().unwrap();
            *num += 1;
        })); // guard drop — мьютекс освобождён
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("итог: {}", *counter.lock().unwrap()); // 10
}"##,
                runnable: true,
            },
            Block::List(&[
                "Arc<T> — как Rc, но счётчик атомарный: безопасен между потоками (небольшая цена за атомарные операции).",
                "Mutex<T> — lock() даёт MutexGuard<T>; через него читают и пишут. Паника внутри — poisoned lock (lock().unwrap() расскажет).",
                "Забыть unlock невозможно: guard освобождается при выходе из области (RAII).",
                "Правило Send/Sync: компилятор проверяет, что тип можно передавать/разделять между потоками.",
            ]),
            Block::Heading("Send и Sync — проверки компилятора"),
            Block::List(&[
                "Send — тип можно переместить в другой поток (большинство типов).",
                "Sync — &T можно разделять между потоками (T: Send для всех ссылок).",
                "Rc не Send — поэтому Rc нельзя в thread::spawn; Arc можно.",
                "Mutex<T>: Send + Sync, если T: Send. RefCell — не Sync: только для одного потока.",
            ]),
            Block::Note("Трейты Send/Sync — авто-трейты: компилятор выводит их сам. Именно они превращают «гонки данных» в ошибку компиляции: send требует T: Send + 'static, и Rc туда просто не пройдёт."),
            Block::Note("RwLock<T> — альтернатива Mutex, когда читателей много: параллельное чтение, эксклюзивная запись."),
        ],
        quiz: vec![
            Question {
                text: "Зачем Mutex, если есть уже Arc?",
                options: &[
                    "Arc даёт владение, Mutex — эксклюзивный доступ к изменяемым данным",
                    "Arc быстрее, Mutex красивее",
                    "Mutex заменяет Arc",
                ],
                answer: 0,
                explain: "Arc решает «сколько владельцев», Mutex решает «кто меняет данные сейчас». Вместе — безопасная разделяемая изменяемость.",
            },
            Question {
                text: "Что вернёт counter.lock()?",
                options: &[
                    "Само значение T",
                    "MutexGuard — временный объект, освобождающий мьютекс при drop",
                    "Булево: удалось или нет",
                ],
                answer: 1,
                explain: "Guard реализует Deref/DerefMut. RAII: покинул область — мьютекс освободился. Забыть unlock нельзя.",
            },
        ],
        exercise: Some(Exercise {
            task: "Соберите результаты работы 4 потоков в общий Arc<Mutex<Vec<u64>>>: каждый поток вычисляет квадрат своего индекса и дописывает в общий вектор. Напечатайте итог.",
            starter: r##"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let results = Arc::new(Mutex::new(vec![]));
    // TODO: 4 потока пишут квадраты индексов
}"##,
            solution: r##"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let results = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..4u64 {
        let results = Arc::clone(&results);
        handles.push(thread::spawn(move || {
            results.lock().unwrap().push(i * i);
        }));
    }
    for h in handles { h.join().unwrap(); }

    println!("{:?}", results.lock().unwrap()); // [0, 1, 4, 9] в произв. порядке
}"##,
        }),
    }
}

fn async_() -> Lesson {
    Lesson {
        id: "conc-async",
        title: "Async/await и Tokio",
        blocks: vec![
            Block::P("async/await — конкурентность без потоков ОС: тысячи «зелёных» задач на нескольких рабочих потоках. Идеально для сетевых сервисов с множеством одновременных соединений."),
            Block::Code {
                caption: "async fn и await",
                code: r##"// Cargo.toml:
// [dependencies]
// tokio = { version = "1", features = ["full"] }

async fn fetch_data(id: u32) -> String {
    // имитация сетевого запроса
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    format!("данные {id}")
}

#[tokio::main]
async fn main() {
    // Последовательно: 200 мс
    let a = fetch_data(1).await;

    // Параллельно: ~100 мс
    let (b, c) = tokio::join!(fetch_data(2), fetch_data(3));

    println!("{a} {b} {c}");
}"##,
                runnable: false,
            },
            Block::List(&[
                "async fn возвращает Future — ленивое описание вычисления.",
                ".await приостанавливает текущую задачу, отдавая поток другим задачам.",
                "Rантайм (Tokio) планирует задачи: #[tokio::main] запускает его для main.",
                "Future — это state machine, скомпилированная из вашего кода.",
            ]),
            Block::Heading("Tokio: задачи и каналы"),
            Block::Code {
                caption: "tokio::spawn — конкурентные задачи",
                code: r##"use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    // spawn возвращает JoinHandle — задача работает на рантайме
    let handle = tokio::spawn(async move {
        for i in 0..3 {
            tx.send(i * 10).await.unwrap();
        }
    });

    while let Some(v) = rx.recv().await {
        println!("получили {v}");
    }
    handle.await.unwrap();
}"##,
                runnable: false,
            },
            Block::Warn("Два правила async-мира: Future ничего не делает до await/spawn (ленивость!). И замыкания/данные для tokio::spawn должны быть 'static + Send — поэтому move и Arc здесь жизненно важны."),
            Block::Note("Выбор: thread::spawn — тяжёлые вычисления (CPU-bound); async — много ожиданий ввода-вывода (сеть, диск). Для блокирующих вычислений внутри async — tokio::task::spawn_blocking."),
            Block::Note("Экосистема async: axum (веб), reqwest (HTTP-клиент), sqlx (БД), tokio::net (сокеты). Наш финальный проект будет использовать axum."),
        ],
        quiz: vec![
            Question {
                text: "Что произойдёт, если создать Future и не сделать await?",
                options: &[
                    "Она выполнится в фоне",
                    "Ничего — Future ленива и не запустится",
                    "Ошибка компиляции",
                ],
                answer: 1,
                explain: "Future — описание работы. Без await/spawn/JoinHandle она не выполняется. Компилятор предупредит об unused.",
            },
            Question {
                text: "Чем async-конкурентность отличается от потоков?",
                options: &[
                    "Ничем",
                    "Задачи кооперативно уступают поток при .await — один поток обслуживает тысячи задач",
                    "async медленнее всегда",
                ],
                answer: 1,
                explain: "Мультиплексирование: async-задачи дешёвые (килобайты стека), переключение в точках await. Для CPU-задач потоки лучше.",
            },
        ],
        exercise: Some(Exercise {
            task: "С помощью tokio::join! выполните три async-функции-задержки параллельно и замерьте время: последовательно было бы ~300 мс, параллельно ~100 мс.",
            starter: r##"// Cargo.toml: tokio = { version = "1", features = ["full"] }
use std::time::{Duration, Instant};

async fn task(ms: u64, name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    format!("{name} готова")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // TODO: join! трёх задач по 100 мс
    println!("время: {:?}", start.elapsed());
}"##,
            solution: r##"use std::time::{Duration, Instant};

async fn task(ms: u64, name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    format!("{name} готова")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        task(100, "A"),
        task(100, "B"),
        task(100, "C"),
    );
    println!("{a}, {b}, {c}");
    println!("время: {:?}", start.elapsed()); // ~100 мс, не 300
}"##,
        }),
    }
}
