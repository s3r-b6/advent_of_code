// https://adventofcode.com/2023/day/6
fn main() {
    let contents = include_str!("../input.txt");
    let mut lines = contents.lines();

    let times_line = lines.next().unwrap()[5..].trim();
    let dists_line = lines.next().unwrap()[9..].trim();

    println!("Part1: {}", part_1(times_line, dists_line));
    println!("Part2: {}", part_2(times_line, dists_line));
}

fn part_1(times_line: &str, dists_line: &str) -> i32 {
    let times: Vec<u32> = times_line
        .split(' ')
        .map(|line| return parse_num(line).unwrap())
        .filter(|n| *n != 0)
        .collect();
    let dists: Vec<u32> = dists_line
        .split(' ')
        .map(|line| return parse_num(line).unwrap())
        .filter(|n| *n != 0)
        .collect();

    let mut acc = 1;
    let mut prev_item_ok = false;
    for idx in 0..times.len() {
        let mut solutions = 0;
        let mut curr_time = times[idx];
        let min_dist = dists[idx];

        let mut curr_speed = 0;
        while curr_time > 0 && prev_item_ok || solutions == 0 {
            if curr_speed * curr_time > min_dist {
                prev_item_ok = true;
                solutions += 1;
            } else {
                prev_item_ok = false;
            }

            curr_speed += 1;
            curr_time -= 1;
        }

        acc *= solutions;
    }

    acc
}

fn part_2(times_line: &str, dists_line: &str) -> u64 {
    let max_time: u64 = parse_u64(&times_line.replace(" ", "")).unwrap() as u64;
    let min_dist: u64 = parse_u64(&dists_line.replace(" ", "")).unwrap() as u64;

    let mut prev_item_ok = false;

    let mut solutions = 0;
    let mut curr_speed = 0;
    let mut curr_time = max_time;

    while curr_time > 0 && prev_item_ok || solutions == 0 {
        if curr_speed * curr_time > min_dist {
            prev_item_ok = true;
            solutions += 1;
        } else {
            prev_item_ok = false;
        }

        curr_speed += 1;
        curr_time -= 1;
    }

    solutions
}

// For the second part an u64 is needed, else, it throws because of overflow in the multiplication
fn parse_u64(num: &str) -> Option<u64> {
    let mut num_buff: u64 = 0;

    let mut pos = num.len() as u64;
    for ch in num.chars() {
        if let '0'..='9' = ch {
            let n = ch as u8 - b'0';
            num_buff += n as u64 * (10_u64).pow((pos - 1) as u32);
            pos -= 1;
        } else {
            return None;
        }
    }

    Some(num_buff)
}

fn parse_num(num: &str) -> Option<u32> {
    let mut num_buff: u32 = 0;

    let mut pos = num.len() as u32;
    for ch in num.chars() {
        if let '0'..='9' = ch {
            let n = ch as u8 - b'0';
            num_buff += n as u32 * (10_u32).pow(pos - 1);
            pos -= 1;
        } else {
            return None;
        }
    }

    Some(num_buff)
}
