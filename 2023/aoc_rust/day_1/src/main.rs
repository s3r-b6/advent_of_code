// https://adventofcode.com/2023/day/1
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Try to open the file from root
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut line = String::with_capacity(10);

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    loop {
        let val = buf_reader.read_line(&mut line);
        if val.is_err() || val.unwrap() == 0 {
            break;
        }
        result_p1 += parse_linedata_p1(&line);
        result_p2 += parse_linedata_p2(&line);

        line.clear();
    }

    println!("P1 Result:{}", result_p1);
    println!("P2 Result:{}", result_p2);
}

#[derive(Debug)]
enum Dir {
    F,
    B,
}

// move the cursor towards Dir, if characters match, continue, else restore pointer and return false
fn match_next(
    chars: &Vec<char>,
    pos: &mut usize,
    match_chars: &[char],
    val: char,
    dir: Dir,
) -> Option<char> {
    let original_pos = *pos;

    for i in 0..match_chars.len() {
        let match_ch = match dir {
            Dir::F => {
                *pos += 1;
                match_chars[i]
            }
            Dir::B => {
                *pos -= 1;
                match_chars[match_chars.len() - (i + 1)]
            }
        };

        let next = chars[*pos];
        if next == match_ch {
            continue;
        }

        *pos = original_pos;
        return None;
    }

    return Some(val);
}

// Not as "clean" as I had in mind, but I kind of like the idea
fn parse_linedata_p2(line: &String) -> u32 {
    let chars: Vec<char> = line.chars().collect();

    let mut first_digit = None;
    let mut second_digit = None;

    let mut front_ptr = 0;
    let mut back_ptr = chars.len() - 1;

    loop {
        let front_char = chars[front_ptr];
        let back_char = chars[back_ptr];

        if first_digit.is_none() {
            first_digit =
                match front_char {
                    '0'..='9' => Some(front_char),
                    'o' => match_next(&chars, &mut front_ptr, &['n', 'e'], '1', Dir::F),
                    't' => match_next(&chars, &mut front_ptr, &['w', 'o'], '2', Dir::F).or(
                        match_next(&chars, &mut front_ptr, &['h', 'r', 'e', 'e'], '3', Dir::F),
                    ),
                    'f' => match_next(&chars, &mut front_ptr, &['o', 'u', 'r'], '4', Dir::F).or(
                        match_next(&chars, &mut front_ptr, &['i', 'v', 'e'], '5', Dir::F),
                    ),
                    's' => match_next(&chars, &mut front_ptr, &['i', 'x'], '6', Dir::F).or(
                        match_next(&chars, &mut front_ptr, &['e', 'v', 'e', 'n'], '7', Dir::F),
                    ),
                    'e' => match_next(&chars, &mut front_ptr, &['i', 'g', 'h', 't'], '8', Dir::F),
                    'n' => match_next(&chars, &mut front_ptr, &['i', 'n', 'e'], '9', Dir::F),
                    _ => None,
                }
        }

        if second_digit.is_none() {
            second_digit =
                match back_char {
                    '0'..='9' => Some(back_char),
                    'o' => match_next(&chars, &mut back_ptr, &['t', 'w'], '2', Dir::B),
                    'e' => match_next(&chars, &mut back_ptr, &['o', 'n'], '1', Dir::B).or(
                        match_next(&chars, &mut back_ptr, &['t', 'h', 'r', 'e'], '3', Dir::B).or(
                            match_next(&chars, &mut back_ptr, &['n', 'i', 'n'], '9', Dir::B).or(
                                match_next(&chars, &mut back_ptr, &['f', 'i', 'v'], '5', Dir::B),
                            ),
                        ),
                    ),
                    'x' => match_next(&chars, &mut back_ptr, &['s', 'i'], '6', Dir::B),
                    'n' => match_next(&chars, &mut back_ptr, &['s', 'e', 'v', 'e'], '7', Dir::B),
                    't' => match_next(&chars, &mut back_ptr, &['e', 'i', 'g', 'h'], '8', Dir::B),
                    'r' => match_next(&chars, &mut back_ptr, &['f', 'o', 'u'], '4', Dir::B),
                    _ => None,
                }
        }

        if first_digit.is_some() && second_digit.is_some() {
            // 0-9 are placed next to each other
            let d1 = first_digit.unwrap() as u32 - '0' as u32;
            let d2 = second_digit.unwrap() as u32 - '0' as u32;

            return (d1 * 10) + d2;
        }

        if back_ptr == 0 || front_ptr == line.len() - 1 {
            break;
        }

        front_ptr += 1;
        back_ptr -= 1;
    }

    unreachable!("This should not happen!")
}

fn parse_linedata_p1(line: &String) -> u32 {
    let chars: Vec<char> = line.chars().collect();

    let mut first_digit = None;
    let mut second_digit = None;

    for i in 0..chars.len() {
        let front_char = chars[i];
        let back_char = chars[chars.len() - (i + 1)];

        if first_digit.is_none() {
            match front_char {
                '0'..='9' => first_digit = Some(front_char),
                _ => {}
            }
        }

        if second_digit.is_none() {
            match back_char {
                '0'..='9' => second_digit = Some(back_char),
                _ => {}
            }
        }

        if first_digit.is_some() && second_digit.is_some() {
            // 0-9 are placed next to each other
            let d1 = first_digit.unwrap() as u32 - '0' as u32;
            let d2 = second_digit.unwrap() as u32 - '0' as u32;

            return (d1 * 10) + d2;
        }
    }

    unreachable!("This should not happen");
}
