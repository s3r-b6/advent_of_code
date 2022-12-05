use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open(r"D:\Rust\advent_of_code\day_2\src\rounds.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut enemy_choices: HashMap<String, i32> = HashMap::new();
    enemy_choices.insert("A".to_string(), 1);
    enemy_choices.insert("B".to_string(), 2);
    enemy_choices.insert("C".to_string(), 3);

    let mut user_choices: HashMap<String, i32> = HashMap::new();
    user_choices.insert("X".to_string(), 1);
    user_choices.insert("Y".to_string(), 2);
    user_choices.insert("Z".to_string(), 3);

    let mut cache = 0;
    for line in buf_reader.lines() {
        let unwrapped_line = line.unwrap();
        let enemy_value = enemy_choices.get(&unwrapped_line[0..1]).unwrap();
        let user_value = user_choices.get(&unwrapped_line[2..3]).unwrap();

        if (enemy_value + 1) % 3 == user_value % 3 {
            println!("the user wins");
            println!("{}vs{} \n", enemy_value, user_value);
        } else {
            if enemy_value == user_value {
                println!("draw");
                println!("{}vs{} \n", enemy_value, user_value);
            } else {
                println!("the enemy wins");
                println!("{}vs{} \n", enemy_value, user_value);
            }
        }
    }
}
