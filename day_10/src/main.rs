fn main() {
    let input =
        std::fs::read_to_string("D:\\rust\\advent_of_code\\day_10\\src\\input.txt").unwrap();
    let searched_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut cycle_count = 0;
    let mut x = 1;
    let mut value_sum = 0;

    let mut row = String::from("");

    for line in input.lines() {
        let (instr, num) = line.split_once(" ").unwrap_or_else(|| (line, ""));
        match instr {
            "noop" => {
                tick(&mut cycle_count, &searched_cycles, &mut value_sum, x);
                draw_crt(&mut row, x);
            }
            "addx" => {
                for i in 0..2 {
                    tick(&mut cycle_count, &searched_cycles, &mut value_sum, x);
                    draw_crt(&mut row, x);
                    if i == 1 {
                        x += num.parse::<i32>().unwrap();
                    }
                }
            }
            _ => panic!("should not happen"),
        }
    }
    println!("Total: {}", value_sum);
}

fn tick(cycle_count: &mut i32, searched_cycles: &Vec<i32>, value_sum: &mut i32, x: i32) {
    *cycle_count += 1;
    if searched_cycles.contains(&*cycle_count) {
        *value_sum += x * *cycle_count;
    }
}

fn draw_crt(row: &mut String, x: i32) {
    if row.len() == 40 {
        println!("{}", row);
        *row = String::from("");
    } else {
        *row += {
            //row_length is current pos?
            let row_length = row.len() as i32;
            if x == row_length || x + 1 == row_length || x - 1 == row_length {
                "#"
            } else {
                "."
            }
        }
    }
}
