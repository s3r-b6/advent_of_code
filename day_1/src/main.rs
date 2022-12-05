use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file =
        File::open(r"D:\Rust\advent_of_code\day_1\src\inventory.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut elf_list: HashMap<String, i32> = HashMap::new();
    let mut value = 0;
    let mut counter = 0;
    for line in buf_reader.lines() {
        if !line.is_err() {
            let unwraped_line = line.unwrap();
            if unwraped_line != "" {
                value += unwraped_line.parse::<i32>().unwrap();
            } else {
                counter += 1;
                elf_list.insert("elf ".to_owned() + &counter.to_string(), value);
                value = 0;
            }
        }
    }

    let mut max_value: (String, i32) = ("".to_string(), 0);
    if !elf_list.is_empty() {
        for elf in elf_list {
            if max_value.1 < elf.1 {
                max_value = (elf.0, elf.1)
            };
        }
        println!(
            "\n\nMax_value is: {}, carried by {}\n\n",
            max_value.1, max_value.0
        );
    } else {
        println!("\n\nerror:  no values were stored\n\n");
    }
}
