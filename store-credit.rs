#![feature(io)]
use std::old_io;

#[derive(Debug)]
struct Problem {
    credit: u32,
    num_items: u32,
    items: Vec<u32>,
}

/// Find two items in Problem whose prices sum to the credit available
fn solve_problem(problem: &Problem) -> (u32, u32) {
    for x in 0..problem.num_items {
        let item_cost = problem.items[x as usize];
        if item_cost > problem.credit {
            continue;
        }

        let target = problem.credit - item_cost;

        for y in (x+1)..problem.num_items {
            let inner_item_cost = problem.items[y as usize];
            if inner_item_cost == target {
                return (x, y);
            }
        }
    };
    unreachable!()
}

fn convert_str_vec(str_vec: Vec<&str>) -> Vec<u32> {
    let mut int_vec = Vec::new();
    for x in &str_vec {
        let parse_result: Result<u32, _> = x.parse();
        let entry = match parse_result {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        };
        int_vec.push(entry);
    }

    int_vec

}

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

fn get_vec() -> Vec<u32> {
    let line: String = get_line();
    let str_prices: Vec<&str> = line.trim().split(' ').collect();
    convert_str_vec(str_prices)
}

fn read_problem() -> Problem {
    let credit = get_int();
    let num_items = get_int();
    let items = get_vec();

    Problem {credit: credit, num_items: num_items, items: items}
}


fn main() {
    let n = get_int();

    for i in 0..n {
        let problem = read_problem();
        let (x, y) = solve_problem(&problem);

        println!("Case #{}: {} {}", i + 1, x + 1, y + 1);
    }
}
