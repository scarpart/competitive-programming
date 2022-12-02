use std::io::{prelude::*, BufReader};
use std::fs::File;

fn highest_calories_elf(filename: &str) -> i64 {
    let file = File::open(filename).expect("file couldn't be opened");
    let reader = BufReader::new(file);

    let mut elf_count = 0;
    let mut highest: [i64; 3] = [0; 3];

    for line in reader.lines() {
        if let Ok(n) = line {
            if n != "" {
                elf_count += n.parse::<i64>().unwrap();
            } else {
                elf_count = 0;
            }
        }
        
        if elf_count > highest[2] && elf_count <= highest[1] {
            highest[2] = elf_count;
        } else if elf_count > highest[1] && elf_count <= highest[0] {
            let tmp = highest[1];
            highest[1] = elf_count;
            highest[2] = tmp;
        } else if elf_count > highest[0] {
            let tmp = highest[0];
            highest[0] = elf_count;
            highest.swap(1, 2);
            highest[1] = tmp;
        }
    }

    let mut combined: i64 = 0;
    for i in highest {
        combined += i;
    }
    combined
}

fn main() {
    let result: i64 = highest_calories_elf("input");
    println!("{}", result);
}