use core::panic;

fn main() {
    let input = std::fs::read_to_string("D:\\Rust\\advent_of_code\\day_3\\src\\items.txt").unwrap();
    let total_priority: u32 = input.lines().map(|line| get_line_priority(line)).sum();
    println!("Total priority: {}", total_priority);
    let mut curr_elf_group = String::new();
    let mut counter = 0;
    let mut badges_priority = 0;
    for elf in input.lines() {
        if counter < 3 {
            curr_elf_group = curr_elf_group + elf + "\n";
            counter += 1;
        } else {
            badges_priority += get_badges_priority(&curr_elf_group);
            curr_elf_group = elf.to_string() + "\n";
            counter = 1;
        }
    }
    //the last group is not included because it is the first line of a new group what calls the function
    badges_priority += get_badges_priority(&curr_elf_group);
    println!("Total badges priority: {}", badges_priority);
}

fn get_char_priority(char_ascii: u32) -> u32 {
    if char_ascii > 64 && char_ascii < 91 {
        char_ascii - 38
    } else if char_ascii > 96 && char_ascii < 123 {
        char_ascii - 96
    } else {
        panic!("should not happen (A-Z, a-z)")
    }
}

//takes a line and returns the sum of repeated values' priority in the line
fn get_line_priority(line: &str) -> u32 {
    let (compart_a, compart_b) = line.split_at(line.len() / 2);
    let mut line_priority = 0;
    let mut already_found: Vec<char> = vec![];

    //test char_a against every single char_b unless it is in the list already_found
    for char_a in compart_a.chars() {
        for char_b in compart_b.chars() {
            if already_found.contains(&char_a) || !char_a.eq(&char_b) {
                continue;
            }
            //println!("repeated_char: {}", char_a);
            let char_priority = get_char_priority(char_a as u32);
            already_found.push(char_a);
            line_priority += char_priority;
        }
    }
    //println!("Line priority: {}", line_priority);
    return line_priority;
}

fn get_badges_priority(line: &str) -> u32 {
    let mut string_list: Vec<String> = vec![];
    let mut found_list: Vec<char> = vec![];
    for elf in line.split("\n") {
        string_list.push(elf.to_string());
    }

    for char_a in string_list[0].chars() {
        if !found_list.contains(&char_a) {
            for char_b in string_list[1].chars() {
                if !found_list.contains(&char_a) && char_a == char_b {
                    for char_c in string_list[2].chars() {
                        if !found_list.contains(&char_a) && char_b == char_c {
                            found_list.push(char_a);
                            return get_char_priority(char_a as u32);
                        }
                    }
                }
            }
        }
    }
    panic!("all groups should have a badge");
}
