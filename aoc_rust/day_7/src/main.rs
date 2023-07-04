use std::collections::HashMap;

const TOTAL_SPACE: u32 = 70000000;
const NEEDED_SPACE: u32 = 30000000;
const MAX_SIZE: u32 = 100000;

//So far, the day that has given me the most problems. Very interesting though.
//I first tried to create a "filesystem" using structs and pointers, but ran into lifetime issues, that I do not really understand yet.
//Then I started working on this solution. The biggest difference and upgrade in this version has been having the current_directory
//as a vector of strings; instead of using a string, popping and pushing and then using join("/") makes this very very simple.

fn main() {
    let input = std::fs::read_to_string("D:\\Rust\\advent_of_code\\day_7\\src\\input.txt").unwrap();

    let mut existing_files: HashMap<String, u32> = HashMap::new();
    let mut existing_directories: HashMap<String, u32> = HashMap::new();
    let mut current_directory: Vec<String> = vec![];

    for line in input.lines() {
        if line.starts_with("$ cd /") {
            current_directory = vec!["root".to_string()];
            if existing_directories.get("root").is_none() {
                existing_directories.insert("root".to_string(), 0);
            }
        } else if line.starts_with("$ cd ..") {
            current_directory.pop();
        } else if line.starts_with("$ cd ") {
            let new_dir = line.replace("$ cd ", "");
            current_directory.push(new_dir);
            if existing_directories
                .get(current_directory.join("/").as_str())
                .is_none()
            {
                existing_directories.insert(current_directory.join("/"), 0);
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let (file_size, name) = line.split_at(line.find(" ").unwrap());
            let current_file_path = current_directory.join("/") + name.trim();
            //if it is new file, register it and add to the size to all parent directories
            if existing_files.get(current_file_path.as_str()).is_none() {
                existing_files.insert(current_file_path, file_size.parse().unwrap());

                //sum recursively
                for i in 0..current_directory.len() {
                    existing_directories
                        .entry(current_directory[0..current_directory.len() - i].join("/"))
                        .and_modify(|size| *size += file_size.parse::<u32>().unwrap());
                }
            }
        }
    }

    let small_files_total: u32 = existing_directories
        .values()
        .filter(|&&size| size <= MAX_SIZE)
        .sum();

    println!("Total size: {}", small_files_total);

    let total_occupied: u32 = *existing_directories.get("root").unwrap();
    let free_space = TOTAL_SPACE - total_occupied;
    let needed_space = NEEDED_SPACE - free_space;

    println!("Total occupied: {}", total_occupied);
    println!("Free space: {}, needed space: {}", free_space, needed_space);

    let mut deleting_candidates: Vec<&u32> = existing_directories
        .values()
        .filter(|&size| size >= &needed_space)
        .collect();

    deleting_candidates.sort();
    println!("Lowest possible file to delete: {}", deleting_candidates[0]);
}
