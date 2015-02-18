use std::old_io;

pub fn get_line() -> String {
    old_io::stdin().read_line().ok().expect("Failed to read line.")
}

pub fn get_int() -> u32 {
    let in_str = get_line();
    let parse_result: Result<u32, _> = in_str.trim().parse();

    match parse_result {
        Ok(num) => num,
        Err(_) => panic!(),
    }
}
