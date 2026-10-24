//! Модуль 8: Умные указатели.

use super::{Block, Exercise, Lesson, Question, Section};

pub fn section() -> Section {
    Section {
        emoji: "🎁",
        title: "Умные указатели",
        blurb: "Box для кучи, Rc для совместного владения, RefCell для внутренней изменяемости.",
        lessons: vec![box_deref(), rc_refcell()],
    }
}

fn box_deref() -> Lesson {
    Lesson {
        id: "smart-box",
        title: "Box и Deref",
        blocks: vec![
            Block::P("Box<T> — умный указатель, размещающий значение в куче. Сам Box маленький (указатель в стеке), а данные — в куче. Когда Box выходит из области видимости, он освобождает данные — это уже знакомые вам правила владения."),
            Block::Code {
                caption: "Box: куча вместо стека",
                code: r##"fn main() {
    // Большое значение — не в стеке, а в куче
    let big = Box::new([0u8; 10_000]);
    println!("размер Box в стеке: {} байт", std::mem::size_of_val(&big));
    println!("размер данных: {} байт", std::mem::size_of_val(&*big));

    // Box ведёт себя как обычное значение (Deref)
    let x = Box::new(5);
    println!("{} {}", *x, x + 1); // разыменование и авто-deref
}"##,
                runnable: true,
            },
            Block::Heading("Три сценария для Box"),
            Block::Code {
                caption: "1. Рекурсивный тип — иначе размер неизвестен",
                code: r##"enum List {
    // Cons(i32, List),        // ОШИБКА: рекурсивный размер
    Cons(i32, Box<List>),      // ок: Box имеет фиксированный размер
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let _ = list;
}"##,
                runnable: false,
            },
            Block::Code {
                caption: "2. Трейт-объекты и 3. Большие данные",
                code: r##"trait Draw {
    fn draw(&self) -> String;
}

struct Button;
struct Canvas;

impl Draw for Button {
    fn draw(&self) -> String { "[кнопка]".into() }
}
impl Draw for Canvas {
    fn draw(&self) -> String { "(холст)".into() }
}

fn main() {
    // Трейт-объект: разные типы под одним указателем
    let widgets: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(Canvas)];
    for w in &widgets {
        println!("{}", w.draw());
    }
}"##,
                runnable: true,
            },
            Block::Heading("Deref и Drop — механика умных указателей"),
            Block::Code {
                caption: "Свой умный указатель",
                code: r##"use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn print_str(s: &str) {
    println!("{s}");
}

fn main() {
    let b = MyBox::new(String::from("привет"));
    // deref coercion: &MyBox<String> -> &String -> &str
    print_str(&b);
}"##,
                runnable: true,
            },
            Block::Note("Deref-приведение (deref coercion) — почему &String подходит там, где ждут &str, а &Box<T> — где ждут &T. Это убирает 90% шумных .as_str()/.as_deref() из кода."),
            Block::Note("Drop — трейт деструктора: fn drop(&mut self). Реализуйте его для освобождения ресурсов (файлы, сокеты). Вызов происходит детерминированно при выходе владельца из области видимости."),
        ],
        quiz: vec![
            Question {
                text: "Почему рекурсивный enum требует Box?",
                options: &[
                    "Для скорости",
                    "Размер типа должен быть известен при компиляции; Box имеет фиксированный размер указателя",
                    "Box включает сборку мусора",
                ],
                answer: 1,
                explain: "Без Box компилятор не может вычислить размер: Cons содержит List, который содержит Cons... Box разрывает цикл фиксированным размером.",
            },
            Question {
                text: "Что такое deref coercion?",
                options: &[
                    "Автоматическое преобразование &SmartPointer<T> -> &T при передаче аргументов",
                    "Удаление указателя вручную",
                    "Копирование данных из кучи",
                ],
                answer: 0,
                explain: "Если T: Deref<Target=U>, то &T автоматически приводится к &U — так &String становится &str.",
            },
        ],
        exercise: Some(Exercise {
            task: "Реализуйте enum Tree { Leaf(i32), Node(Box<Tree>, Box<Tree>) } и функцию sum(&Tree), суммирующую все листья. Соберите дерево 1 + (2 + 3) = 6.",
            starter: r##"enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),
}

fn sum(tree: &Tree) -> i32 {
    // TODO: match по вариантам
    0
}

fn main() {
    let tree = Tree::Node(
        Box::new(Tree::Leaf(1)),
        Box::new(Tree::Node(Box::new(Tree::Leaf(2)), Box::new(Tree::Leaf(3)))),
    );
    println!("{}", sum(&tree)); // 6
}"##,
            solution: r##"enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),
}

fn sum(tree: &Tree) -> i32 {
    match tree {
        Tree::Leaf(v) => *v,
        Tree::Node(l, r) => sum(l) + sum(r),
    }
}

fn main() {
    let tree = Tree::Node(
        Box::new(Tree::Leaf(1)),
        Box::new(Tree::Node(Box::new(Tree::Leaf(2)), Box::new(Tree::Leaf(3)))),
    );
    println!("{}", sum(&tree));
}"##,
        }),
    }
}

fn rc_refcell() -> Lesson {
    Lesson {
        id: "smart-rc",
        title: "Rc и RefCell: совместное владение",
        blocks: vec![
            Block::P("Box даёт одного владельца. А если значение нужно нескольким владельцам одновременно (граф, дерево с общими узлами)? Rc<T> — счётчик ссылок: значение живёт, пока есть хоть один Rc. Clone у Rc не копирует данные, а увеличивает счётчик."),
            Block::Code {
                caption: "Rc — reference counting",
                code: r##"use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("общие данные"));
    println!("владельцев: {}", Rc::strong_count(&shared)); // 1

    let a = Rc::clone(&shared);  // +1, копии данных нет
    let b = Rc::clone(&shared);  // +1
    println!("владельцев: {}", Rc::strong_count(&shared)); // 3

    drop(b);                     // -1
    println!("владельцев: {}", Rc::strong_count(&shared)); // 2

    println!("{} / {a}", shared.to_uppercase());
} // счётчик до нуля — данные освобождаются
"##,
                runnable: true,
            },
            Block::Warn("Rc — только для однопоточного кода и только для чтения: из Rc<T> нельзя получить &mut T. Для многопоточности — Arc (atomic reference counting). Мутабельность поверх Rc даёт RefCell."),
            Block::Heading("RefCell: внутренняя изменяемость"),
            Block::P("RefCell<T> переносит правила заимствования из компиляции в рантайм: он хранит счётчик заимствований и паникует, если нарушить «много читателей ИЛИ один писатель». Это позволяет менять данные через неизменяемую ссылку — легально и безопасно (паника вместо UB)."),
            Block::Code {
                caption: "RefCell — borrow в рантайме",
                code: r##"use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    //borrow_mut — изменяемый заём в рантайме
    data.borrow_mut().push(4);
    data.borrow_mut().sort();
    println!("{:?}", data.borrow()); // [1, 2, 3, 4]

    // Нарушение правила ловится в рантайме:
    let r1 = data.borrow();
    // let r2 = data.borrow_mut(); // паника: already borrowed
    println!("{:?}", r1);
}"##,
                runnable: true,
            },
            Block::Heading("Классическая комбинация: Rc<RefCell<T>>"),
            Block::Code {
                caption: "Несколько владельцев, изменяемые данные",
                code: r##"use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let leaf = Rc::new(RefCell::new(Node { value: 3, children: vec![] }));
    let mut root_children = Vec::new();
    root_children.push(Rc::clone(&leaf));

    let root = Node { value: 1, children: root_children };

    // Меняем leaf через root-структуру — Rc+RefCell позволяет
    leaf.borrow_mut().value = 30;

    println!("{:?}", root.children[0].borrow().value); // 30
}"##,
                runnable: true,
            },
            Block::List(&[
                "Box<T> — один владелец, правила заимствования при компиляции.",
                "Rc<T> — много владельцев, только чтение, однопоточно.",
                "Arc<T> — много владельцев, потоки (атомарный счётчик).",
                "RefCell<T> — изменяемость в рантайме (паника при нарушении).",
                "Rc<RefCell<T>> — многовладельческие изменяемые данные; Mutex заменяет RefCell в потоках.",
            ]),
            Block::Note("Цикл ссылок Rc -> Rc никогда не освободится (счётчик не упадёт до нуля) — это утечка памяти. Решение: std::rc::Weak для «слабых» обратных ссылок."),
        ],
        quiz: vec![
            Question {
                text: "Что происходит при Rc::clone(&x)?",
                options: &[
                    "Данные копируются в новую память",
                    "Увеличивается счётчик ссылок, данные общие",
                    "Создаётся глубокая копия",
                ],
                answer: 1,
                explain: "Rc::clone дёшев: только +1 к счётчику. Данные освобождаются, когда счётчик обнулится.",
            },
            Question {
                text: "Когда RefCell паникует?",
                options: &[
                    "Никогда",
                    "При нарушении правил заимствования в рантайме: одновременно mut-заём и другой заём",
                    "При превышении размера кучи",
                ],
                answer: 1,
                explain: "RefCell переносит проверку borrow-правил из компиляции в рантайм. Нарушение — предсказуемая паника, а не UB.",
            },
        ],
        exercise: Some(Exercise {
            task: "Создайте Rc<RefCell<Vec<String>>>, добавьте в него элементы из двух разных переменных и напечатайте итоговый список вместе со счётчиком владельцев.",
            starter: r##"use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // TODO
}"##,
            solution: r##"use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let log = Rc::new(RefCell::new(Vec::new()));

    let a = Rc::clone(&log);
    let b = Rc::clone(&log);

    a.borrow_mut().push(String::from("первое"));
    b.borrow_mut().push(String::from("второе"));

    println!("владельцев: {}", Rc::strong_count(&log)); // 3
    println!("{:?}", log.borrow()); // ["первое", "второе"]
}"##,
        }),
    }
}
