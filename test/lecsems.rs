/*// Определяем структуру
struct Point {
    x: i32,
    y: i32,
}

// Реализуем методы для структуры
impl Point {
    // Конструктор
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    // Метод для вычисления расстояния до другой точки
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx * dx + dy * dy) as f64).sqrt()
    }
}

// Перечисление
enum Direction {
    North,
    South,
    East,
    West,
}
// Функция, которая использует сопоставление с образцом
fn get_direction_string(dir: Direction) -> &'static str {
    match dir {
        Direction::North => "North",
        Direction::South => "South",
        Direction::East => "East",
        Direction::West => "West",
    }
}
*/

// Основная функция
fn main() {
    // Переменные и типы данных
    let mut x = 5;
    let y = 10;
    let z: f64 = 3.14;
    let is_true = true;
    let name = "Rust";

    // Условные операторы
    if x < y {
        println!("x is less than y");
    } else if x > y {
        println!("x is greater than y");
    } else {
        println!("x is equal to y");
    }

    // Циклы
    for i in 0..5 {
        println!("i = {}", i);
    }

    let mut counter = 0;
    while counter < 5 {
        println!("counter = {}", counter);
        counter += 1;
    }

    // Работа с векторами
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);
    println!("Vector: {:?}", vec);

    // Работа с Option
    let some_value = Some(10);
    if let Some(value) = some_value {
        println!("Some value: {}", value);
    }

    // Работа с Result
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Использование структуры и методов
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    println!("Distance between p1 and p2: {}", p1.distance_to(&p2));

    // Использование перечисления
    let dir = Direction::North;
    println!("Direction: {}", get_direction_string(dir));

    // Замыкания
    let add = |a: i32, b: i32| a + b;
    println!("5 + 10 = {}", add(5, 10));

    // Работа с строками
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    // Приведение типов
    let int_to_float = x as f64;
    println!("int_to_float: {}", int_to_float);

    // Макросы
    println!("This is a macro: {:?}", vec);

    // Работа с кортежами
    let tuple = (1, "hello", 3.14);
    println!("Tuple: {:?}", tuple);

    // Работа с срезами
    let slice = &vec[1..3];
    println!("Slice: {:?}", slice);

    // Работа с ссылками и заимствованием
    let ref_to_x = &x;
    println!("ref_to_x: {}", ref_to_x);

    // Работа с изменяемыми ссылками
    let mut mutable_x = x;
    let ref_to_mutable_x = &mut mutable_x;
    *ref_to_mutable_x += 1;
    println!("mutable_x after increment: {}", mutable_x);

    // Работа с функциями высшего порядка
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    // Работа с трейтами
    trait Printable {
        fn print(&self);
    }

    impl Printable for Point {
        fn print(&self) {
            println!("Point({}, {})", self.x, self.y);
        }
    }

    let p = Point::new(10, 20);
    p.print();
}
