#![feature(core)]
#![feature(io)]
use std::str;

mod codejam_io;

fn map_char(c: char) -> String {
    match c {
        'a' => "2",
        'b' => "22",
        'c' => "222",
        'd' => "3",
        'e' => "33",
        'f' => "333",
        'g' => "4",
        'h' => "44",
        'i' => "444",
        'j' => "5",
        'k' => "55",
        'l' => "555",
        'm' => "6",
        'n' => "66",
        'o' => "666",
        'p' => "7",
        'q' => "77",
        'r' => "777",
        's' => "7777",
        't' => "8",
        'u' => "88",
        'v' => "888",
        'w' => "9",
        'x' => "99",
        'y' => "999",
        'z' => "9999",
        ' ' => "0",
        _ => panic!(),
    }.to_string()
}

fn replace_pipes(input: String) -> String {
    // Replace pipes with spaces where necessary and remove
    // the rest.
    let mut input_bytes = input.as_bytes().to_vec();

    for i in 0..input_bytes.len() {
        if input_bytes[i] == b'|' {
            if input_bytes[i-1] == input_bytes[i+1] {
                input_bytes[i] = b' ';
            }
        }
    }

    let parsed_result: Result<&str, _> = str::from_utf8(input_bytes.as_slice());

    let parsed = match parsed_result {
        Ok(s) => s,
        Err(_) => panic!(),
    };

    parsed.replace("|", "")

}


fn map_string(input: &str) -> String {
    let mut output_vec: Vec<String> = Vec::new();

    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        output_vec.push(map_char(c));
    }

    let piped = output_vec.connect("|");

    replace_pipes(piped)
}

fn main() {
    let n = codejam_io::get_int();

    for i in 0..n {
        let line = codejam_io::get_line();
        println!("Case #{}: {}", i + 1, map_string(&line));
    }
}
