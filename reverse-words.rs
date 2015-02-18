#![feature(io)]
#![feature(collections)]
use std::old_io;

fn get_line() -> String {
    old_io::stdin().read_line().ok().expect("Failed to read line.")
}

fn get_int() -> u32 {
    let in_str = get_line();
    let parse_result: Result<u32, _> = in_str.trim().parse();

    match parse_result {
        Ok(num) => num,
        Err(_) => panic!(),
    }
}

fn get_words() -> Vec<String> {
    let line: String = get_line();
    let mut words: Vec<&str> = line.trim().split(' ').collect();
    let mut string_words: Vec<String> = Vec::with_capacity(words.len());
    words.reverse();

    for x in &words {
        string_words.push(String::from_str(x));
    }

    string_words
}

fn main() {
    let n = get_int();

    for i in 0..n {
        println!("Case #{}: {}", i + 1, get_words().connect(" "));
    }
}
