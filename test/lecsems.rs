
// Основная функция
fn main() -> i32 {
    if x > y {
        if x > y {
            println!("x is greater than y");
        } else if x < y {
            println!("y is greater than x");
        } else {
            println!("y is = x");
        }
    } else if x < y {
        println!("y is greater than x");
    } else {
        println!("y is = x");
    }
}
