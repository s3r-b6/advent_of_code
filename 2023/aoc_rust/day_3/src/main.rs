// https://adventofcode.com/2023/day/3
fn main() {
    let mut parts_map: Vec<Vec<char>> = vec![];
    let contents = include_str!("../input.txt");

    for l in contents.split('\n') {
        if !l.is_empty() {
            parts_map.push(l.chars().collect())
        }
    }

    let parts_acc = part1(&parts_map);
    let gears_acc = part2(&parts_map);

    println!("Part1: {}", parts_acc);
    println!("Part2: {}", gears_acc);
}

fn is_symbol(ch: char) -> bool {
    match ch {
        '0'..='9' | '.' => return false,
        _ => return true,
    };
}

fn part1(parts_map: &Vec<Vec<char>>) -> u32 {
    let mut num_buff = String::with_capacity(4);
    let mut symbol_seen = false;

    let mut parts_acc = 0;
    for row in 0..parts_map.len() {
        for col in 0..parts_map[row].len() {
            let char = &parts_map[row][col];

            if let '0'..='9' = char {
                if !symbol_seen {
                    symbol_seen = check_directions_p1(row, col, &parts_map);
                }
                num_buff.push(*char);
                continue;
            }

            if !num_buff.is_empty() {
                if symbol_seen {
                    parts_acc += num_buff.parse::<u32>().unwrap();
                    symbol_seen = false;
                }

                num_buff.clear();
            }
        }
    }
    parts_acc
}

fn check_directions_p1(row: usize, col: usize, parts_map: &Vec<Vec<char>>) -> bool {
    let mut result = false;

    // Up, left, down, right
    if !result && row > 0 {
        result = result || is_symbol(parts_map[row - 1][col]);
    };
    if !result && col > 0 {
        result = result || is_symbol(parts_map[row][col - 1]);
    };
    if !result && row < parts_map.len() - 1 {
        result = result || is_symbol(parts_map[row + 1][col]);
    };
    if !result && col < parts_map[row].len() - 1 {
        result = result || is_symbol(parts_map[row][col + 1]);
    };

    // Diagonals
    if !result && row > 0 && col > 0 {
        result = result || is_symbol(parts_map[row - 1][col - 1]);
    };
    if !result && row < parts_map.len() - 1 && col < parts_map[row].len() - 1 {
        result = result || is_symbol(parts_map[row + 1][col + 1]);
    };
    if !result && row < parts_map.len() - 1 && col > 0 {
        result = result || is_symbol(parts_map[row + 1][col - 1]);
    };
    if !result && row > 0 && col < parts_map[row].len() - 1 {
        result = result || is_symbol(parts_map[row - 1][col + 1]);
    };

    return result;
}

fn part2(parts_map: &Vec<Vec<char>>) -> u32 {
    let mut ratio_acc = 0;
    for row in 0..parts_map.len() {
        for col in 0..parts_map[row].len() {
            if parts_map[row][col] == '*' {
                ratio_acc += check_directions_p2(row, col, &parts_map);
            }
        }
    }

    ratio_acc
}

fn check_nums(
    parts_map: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    nums: &mut Vec<u32>,
) -> (bool, bool) {
    if let '0'..='9' = parts_map[row][col] {
        let result = build_num(parts_map, row, col);
        nums.push(result.0);
        return result.1;
    }

    return (false, false);
}

fn build_num(parts_map: &Vec<Vec<char>>, row: usize, col: usize) -> (u32, (bool, bool)) {
    let mut num_buff = String::with_capacity(4);
    num_buff.push(parts_map[row][col]);

    // L and R
    let mut checked = (false, false);

    // Check before
    let mut c = col;
    while c > 0 {
        let char = parts_map[row][c - 1];
        if let '0'..='9' = char {
            num_buff.insert(0, char);
            checked.0 = true;
            c -= 1;
            continue;
        }

        break;
    }

    // Check after
    let mut c = col;
    while c < parts_map[row].len() - 1 {
        let char = parts_map[row][c + 1];
        if let '0'..='9' = char {
            num_buff.push(char);
            checked.1 = true;
            c += 1;
            continue;
        }

        break;
    }

    return (num_buff.parse().unwrap(), checked);
}

////////////
// The code is structured so that every direction in all 8 directions is checked,
// the issue is that a number can "span" between one and 3 directions:
//
// See the case of 35 ->
//
//     467..114..
//     ...*......
//     ..35..633.
//     ......#...
//     617*......
//
//  Here, 35 is both bottom and bottom left, so, 35 would be found twice.
//  If 35 had a 1 to its right, 351 would be found thrice.
//
//  I have solved this simply by having the function that reads the chars
//  and builds up numbers return if it has checked left and/or right.
//  This is most probably not the best solution, but it is the most 'naive'
//  I have come up with no refactor
//
///////
fn check_directions_p2(row: usize, col: usize, parts_map: &Vec<Vec<char>>) -> u32 {
    let mut nums: Vec<u32> = vec![];

    let checked_top = if row > 0 {
        check_nums(&parts_map, row - 1, col, &mut nums)
    } else {
        (false, false)
    };

    let checked_bot = if row < parts_map.len() - 1 {
        check_nums(&parts_map, row + 1, col, &mut nums)
    } else {
        (false, false)
    };

    if col > 0 {
        check_nums(&parts_map, row, col - 1, &mut nums);
    }
    if col < parts_map[row].len() - 1 {
        check_nums(&parts_map, row, col + 1, &mut nums);
    }

    // Diagonals
    if !checked_bot.1 && row < parts_map.len() - 1 && col < parts_map[row].len() - 1 {
        check_nums(&parts_map, row + 1, col + 1, &mut nums);
    }
    if !checked_bot.0 && row < parts_map.len() - 1 && col > 0 {
        check_nums(&parts_map, row + 1, col - 1, &mut nums);
    }
    if !checked_top.0 && row > 0 && col > 0 {
        check_nums(&parts_map, row - 1, col - 1, &mut nums);
    }
    if !checked_top.1 && row > 0 && col < parts_map[row].len() - 1 {
        check_nums(&parts_map, row - 1, col + 1, &mut nums);
    }

    if nums.len() != 2 {
        return 0;
    } else {
        return nums[0] * nums[1];
    }
}
