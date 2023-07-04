use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

//Part1: Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
//Part2: Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

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
    let mut sec_max_value: (String, i32) = ("".to_string(), 0);
    let mut third_max_value: (String, i32) = ("".to_string(), 0);
    if !elf_list.is_empty() {
        for elf in elf_list {
            if third_max_value.1 < elf.1 {
                if sec_max_value.1 < elf.1 {
                    if max_value.1 < elf.1 {
                        third_max_value = sec_max_value;
                        sec_max_value = max_value;
                        max_value = elf;
                    } else {
                        third_max_value = sec_max_value;
                        sec_max_value = elf;
                    }
                } else {
                    third_max_value = elf;
                }
            }
        }
        println!(
            "\n\nMax_value is: {}, carried by {}",
            max_value.1, max_value.0
        );
        println!(
            "Second_max_value is: {}, carried by {}",
            sec_max_value.1, sec_max_value.0
        );
        println!(
            "Third_max_value is: {}, carried by {}",
            third_max_value.1, third_max_value.0
        );
        println!(
            "Together, they are carrying {} cals\n\n",
            third_max_value.1 + sec_max_value.1 + max_value.1
        );
    } else {
        println!("\n\nerror:  no values were stored\n\n");
    }
}
