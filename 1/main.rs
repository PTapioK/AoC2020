use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Brute-force algorithm for AoC 2020 (1) and learning Rust by experimenting
fn main() {

    let mut values: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(ref _ip) = line {
                values.push(line.unwrap().parse().unwrap());
            }
        }
    }
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    for val1 in &values {
        for val2 in &values {
            if val1 + val2 == 2020 && a == 0
            {
                a = *val1;
                b = *val2;
                break;
            }
        }
    }
    println!("{}", a * b);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}