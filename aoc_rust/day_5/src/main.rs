use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("D:\\Rust\\advent_of_code\\day_5\\src\\input.txt").unwrap();
    //first, separate the columns from the instructions:
    let (boxes_column, instructions) = input.split_at(344);
    //vec of 9 vecs
    let mut warehouse: Vec<Vec<&str>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    //recreate the columns in memory:

    //added a space in the input file so that it is always groups of four chars for the columns (without the
    //space the last column is (.{3}\n), all other items could be seen as either (.{4}) or (.{3}\s))
    let column_item_pattern = Regex::new("(.{4})").unwrap();
    for line in boxes_column.lines() {
        let mut counter = 0;
        for column in column_item_pattern.find_iter(line) {
            let curr_column = column.as_str();
            //if a column is an item, or it is empty, add it. This is to leave indexes out, as they are not
            //really useful inside the vec itself
            if curr_column.contains("[") {
                warehouse[counter].insert(0, curr_column);
                counter += 1;
            } else if curr_column.eq("    ") {
                counter += 1;
                continue;
            }
        }
    }

    println!("warehouse before ordering: \n");
    print_warehouse(&warehouse);
    println!("warehouse after ordering: \n");
    //for each line of instructions, call the instruction parser
    for instruction in instructions.lines() {
        parse_instruction(&mut warehouse, &instruction);
    }
    print_warehouse(&warehouse);
}

fn print_warehouse(warehouse: &Vec<Vec<&str>>) {
    let mut counter = 1;
    for column in warehouse {
        print!("col: {counter} => ");
        for item in column {
            print!("{}", item);
        }
        counter += 1;
        print!("\n");
    }
    print!("\nlast line is: ");
    for column in warehouse {
        print!("{}", column.last().unwrap().chars().nth(1).unwrap());
    }
    print!("\n\n");
}

fn parse_instruction(warehouse: &mut Vec<Vec<&str>>, line: &str) {
    let get_indexes = Regex::new("[^a-z\\s]+").unwrap();
    let mut move_number: u16 = 0;
    let mut from_index: u16 = 0;

    // 0 => move || 1 => from || 2 => to
    let mut counter = 0;
    for instruction_match in get_indexes.find_iter(line) {
        let index: u16 = instruction_match.as_str().parse().unwrap();
        if counter == 0 {
            move_number = index;
        } else if counter == 1 {
            from_index = index;
        } else {
            if move_number == 1 {
                let pop_item = warehouse[(from_index - 1) as usize].pop().unwrap();
                warehouse[(index - 1) as usize].push(pop_item);
            } else {
                let mut push_queue: Vec<&str> = vec![];
                for _ in 0..move_number {
                    let pop_item = warehouse[(from_index - 1) as usize].pop().unwrap();
                    push_queue.insert(0, pop_item);
                }
                warehouse[(index - 1) as usize].append(&mut push_queue);
            }
        }
        counter += 1;
    }
}
