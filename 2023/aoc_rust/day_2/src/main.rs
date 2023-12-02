fn main() {
    let contents = include_str!("../input.txt");

    let mut acc_p1 = 0;
    let mut acc_p2 = 0;
    for line in contents.lines() {
        let id_sep = line.find(":").unwrap();
        let game_id = &line[5..id_sep].parse().unwrap();

        if is_valid(&line[id_sep + 1..], 12, 13, 14) {
            acc_p1 += game_id;
        }
        acc_p2 += get_min_numbers(&line[id_sep + 1..]);
    }

    println!("P1:{}", acc_p1);
    println!("P2:{}", acc_p2);
}

fn get_min_numbers(line: &str) -> u32 {
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    for part in line.split(";") {
        let mut already_found_color = false;
        let mut val = String::with_capacity(3);
        for char in part.chars() {
            match (char, already_found_color) {
                ('0'..='9', _) => {
                    already_found_color = false;
                    val.push(char);
                }
                ('r', false) => {
                    already_found_color = true;
                    let value = val.parse().unwrap();
                    min_red = min_red.max(value);
                    val.clear();
                }
                ('g', false) => {
                    already_found_color = true;
                    let value = val.parse().unwrap();
                    min_green = min_green.max(value);
                    val.clear();
                }
                ('b', false) => {
                    already_found_color = true;
                    let value = val.parse().unwrap();
                    min_blue = min_blue.max(value);
                    val.clear();
                }
                _ => continue,
            }
        }
    }

    return min_red * min_blue * min_green;
}

fn is_valid(line: &str, max_red: u8, max_green: u8, max_blue: u8) -> bool {
    let mut val = String::with_capacity(3);
    for part in line.split(";") {
        let mut already_found_color = false;
        for char in part.chars() {
            match (char, already_found_color) {
                (' ', _) | (',', _) => continue,
                ('0'..='9', _) => {
                    already_found_color = false;
                    val.push(char);
                }
                ('r', false) => {
                    already_found_color = true;
                    if val.parse::<u8>().unwrap() > max_red {
                        return false;
                    }
                    val.clear();
                }
                ('g', false) => {
                    already_found_color = true;
                    if val.parse::<u8>().unwrap() > max_green {
                        return false;
                    }
                    val.clear();
                }
                ('b', false) => {
                    already_found_color = true;
                    if val.parse::<u8>().unwrap() > max_blue {
                        return false;
                    }
                    val.clear();
                }
                _ => {}
            }
        }
    }

    return true;
}
