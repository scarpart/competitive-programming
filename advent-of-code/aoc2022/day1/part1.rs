use std::io::{prelude::*, BufReader};
use std::fs::File;

fn highest_calories_elf(filename: &str) -> i64 {
    let file = File::open(filename).expect("file couldn't be opened");
    let reader = BufReader::new(file);

    let mut elf_count = 0;
    let mut highest = 0;

    for line in reader.lines() {
        if let Ok(n) = line {
            if n != "" {
                elf_count += n.parse::<i64>().unwrap();
            } else {
                elf_count = 0;
            }
        }
        
        if elf_count > highest {
            highest = elf_count;
        }
    }

    highest
}

fn main() {
    let result: i64 = highest_calories_elf("input");
    println!("{}", result);
}