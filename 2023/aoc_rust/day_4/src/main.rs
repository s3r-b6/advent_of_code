// https://adventofcode.com/2023/day/4
fn main() {
    let contents = include_str!("../input.txt");
    let lines: Vec<&str> = contents.lines().collect();

    let mut p1: u32 = 0;
    let p2: u32 = process_lines_p2(&lines);
    lines.into_iter().for_each(|l| p1 += process_line_p1(&l));

    println!("P1:{}", p1);
    println!("P2:{}", p2);
}

fn parse_num(num: &str) -> u32 {
    let mut num_buff: u32 = 0;

    let mut pos = num.len() as u32;
    for ch in num.chars() {
        match ch {
            '0'..='9' => {
                let n = ch as u8 - b'0';
                num_buff += n as u32 * (10_u32).pow(pos - 1);
                pos -= 1;
            }
            _ => continue,
        }
    }

    num_buff
}

fn process_line_p1(line: &str) -> u32 {
    return match (line.find(':'), line.find('|')) {
        (Some(start_idx), Some(sep_idx)) => {
            let winning_nums = &line[start_idx + 1..sep_idx];
            let have_nums = &line[sep_idx + 1..];

            // The filter is to get rid of zeros that result of empty string
            // splits
            let win_vec: Vec<u32> = winning_nums
                .split(' ')
                .map(parse_num)
                .filter(|num| *num != 0_u32)
                .collect();
            let have_vec: Vec<u32> = have_nums
                .split(' ')
                .map(parse_num)
                .filter(|num| *num != 0_u32)
                .collect();

            let mut found_one = false;
            let mut pts = 0;
            for el in win_vec {
                if have_vec.contains(&el) {
                    if found_one {
                        pts *= 2;
                    } else {
                        pts += 1;
                    }

                    found_one = true;
                }
            }

            pts
        }
        _ => unreachable!("Should not happen"),
    };
}

fn process_lines_p2(contents: &Vec<&str>) -> u32 {
    let mut total_cards = vec![1_u32; contents.len()];

    for idx in 0..contents.len() {
        let line = contents[idx];

        match (line.find(':'), line.find('|')) {
            (Some(start_idx), Some(sep_idx)) => {
                let winning_nums = &line[start_idx + 1..sep_idx];
                let have_nums = &line[sep_idx + 1..];

                let win_vec: Vec<u32> = winning_nums
                    .split(' ')
                    .map(parse_num)
                    .filter(|num| *num != 0_u32)
                    .collect();

                let have_vec: Vec<u32> = have_nums
                    .split(' ')
                    .map(parse_num)
                    .filter(|num| *num != 0_u32)
                    .collect();

                let mut next_idx = idx + 1;
                for win_el in win_vec {
                    if have_vec.contains(&win_el) {
                        total_cards[next_idx] += total_cards[idx];
                        next_idx += 1;
                    }
                }
            }
            _ => unreachable!("Should not happen"),
        }
    }

    total_cards.into_iter().sum()
}
