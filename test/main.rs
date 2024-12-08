use rand::Rng;
use std::collections::VecDeque;
use std::{
    collections::HashSet,
    io::{self, Write},
};

fn read_from_keyboard() -> u32 {
    io::stdout().flush().expect("flush error");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    if let Ok(i) = trimmed.parse::<u32>() {
        return i;
    } else {
        println!("\x1b[31m!!!No number in input, установлено значение по умолчанию(10)!!!\x1b[0m");
        return 10;
    }
}
