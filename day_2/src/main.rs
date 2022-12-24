use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    get_score_part1();
    get_score_part2();
}

fn get_score_part1() {
    let file = File::open(r"D:\Rust\advent_of_code\day_2\src\rounds.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);
    //Not really necessary to use a Hashmap at all, but thought maybe they would be useful for the second part
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

        //if the next choice to the enemy choice is the users choice, then the user wins
        //it is cyclic, so modular arithmetic is used
        if (enemy_value + 1) % 3 == user_value % 3 {
            cache += user_value + 6;
        } else if enemy_value == user_value {
            cache += user_value + 3;
        } else {
            cache += user_value;
        }
    }
    println!("\nThe user's score for part 1 is: {}", cache);
}

fn get_score_part2() {
    let file = File::open(r"D:\Rust\advent_of_code\day_2\src\rounds.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);
    let mut enemy_choices: HashMap<String, i32> = HashMap::new();
    enemy_choices.insert("A".to_string(), 1);
    enemy_choices.insert("B".to_string(), 2);
    enemy_choices.insert("C".to_string(), 3);

    let mut round_results: HashMap<String, String> = HashMap::new();
    round_results.insert("X".to_string(), "lose".to_string());
    round_results.insert("Y".to_string(), "draw".to_string());
    round_results.insert("Z".to_string(), "win".to_string());

    let mut cache = 0;
    for line in buf_reader.lines() {
        let unwrapped_line = line.unwrap();
        let enemy_value = enemy_choices.get(&unwrapped_line[0..1]).unwrap();
        let round_result = round_results.get(&unwrapped_line[2..3]).unwrap();

        if round_result == "win" {
            let user_value = {
                if enemy_value == &3 {
                    1
                } else {
                    enemy_value + 1
                }
            };
            cache += user_value + 6;
        } else if round_result == "draw" {
            cache += enemy_value + 3;
        } else {
            let user_value = {
                if enemy_value == &1 {
                    3
                } else {
                    enemy_value - 1
                }
            };
            cache += user_value;
        }
    }
    println!("\nThe user's score for part 2 is: {}\n", cache);
}
