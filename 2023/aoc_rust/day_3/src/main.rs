// https://adventofcode.com/2023/day/3
fn main() {
    let mut parts_map: Vec<Vec<char>> = include_str!("../input.txt")
        .split('\n') // Last line is empty because of this
        .map(|l| l.chars().collect())
        .collect();

    parts_map.pop();

    println!("Part1: {}", part1(&parts_map));
    println!("Part2: {}", part2(&parts_map));
}

fn part1(parts_map: &Vec<Vec<char>>) -> u32 {
    let mut num_buff = String::with_capacity(4);
    let mut symbol_seen = false;

    let mut parts_acc = 0;
    for row in 0..parts_map.len() {
        for col in 0..parts_map[row].len() {
            let char = &parts_map[row][col];

            if let '0'..='9' = char {
                symbol_seen = symbol_seen || check_directions_p1(row, col, &parts_map);
                num_buff.push(*char);
                continue; // Keep looking for nums
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

    return parts_acc;
}

// This could be easily rewritten in way less lines, but checking if result has already
// been set (we only care if there is one in any dir) is a easy enough optimization
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

fn is_symbol(ch: char) -> bool {
    match ch {
        '0'..='9' | '.' => return false,
        _ => return true,
    };
}

fn part2(parts_map: &Vec<Vec<char>>) -> u32 {
    let mut ratio_acc = 0;
    for row in 0..parts_map.len() {
        for col in 0..parts_map[row].len() {
            if parts_map[row][col] != '*' {
                continue;
            }

            ratio_acc += check_directions_p2(row, col, &parts_map);
        }
    }

    return ratio_acc;
}

fn check_directions_p2(row: usize, col: usize, parts_map: &Vec<Vec<char>>) -> u32 {
    let mut nums: Vec<u32> = vec![];

    let checked_top = if row > 0 {
        check_for_nums(&parts_map, &mut nums, row - 1, col)
    } else {
        (false, false)
    };

    let checked_bot = if row < parts_map.len() - 1 {
        check_for_nums(&parts_map, &mut nums, row + 1, col)
    } else {
        (false, false)
    };

    if col > 0 {
        check_for_nums(&parts_map, &mut nums, row, col - 1);
    }
    if col < parts_map[row].len() - 1 {
        check_for_nums(&parts_map, &mut nums, row, col + 1);
    }

    // Diagonals
    if !checked_bot.1 && row < parts_map.len() - 1 && col < parts_map[row].len() - 1 {
        check_for_nums(&parts_map, &mut nums, row + 1, col + 1);
    }
    if !checked_bot.0 && row < parts_map.len() - 1 && col > 0 {
        check_for_nums(&parts_map, &mut nums, row + 1, col - 1);
    }
    if !checked_top.0 && row > 0 && col > 0 {
        check_for_nums(&parts_map, &mut nums, row - 1, col - 1);
    }
    if !checked_top.1 && row > 0 && col < parts_map[row].len() - 1 {
        check_for_nums(&parts_map, &mut nums, row - 1, col + 1);
    }

    if nums.len() != 2 {
        return 0;
    }

    return nums[0] * nums[1];
}

// The code is structured so that every direction in all 8 directions is checked,
// the only issue is that a number can "span" between one and 3 directions:
//
//     467..114..
//     ...*......
//     ..35..633.
//     ......#...
//     617*......
//
//  Here, 35 is found in both bottom and bottom left, so, 35 would be found twice.
//  If 35 had a 1 to its right, 351 would be found thrice.
//
//  The bool tuple returned here represents if a number was found to the left and/or
//  to the right.
fn check_for_nums(
    parts_map: &Vec<Vec<char>>,
    nums: &mut Vec<u32>,
    row: usize,
    col: usize,
) -> (bool, bool) {
    // The result is irrelevant if this is true
    if nums.len() > 2 {
        return (false, false);
    }

    if let '0'..='9' = parts_map[row][col] {
        // Build a num
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

        nums.push(num_buff.parse().unwrap());
        return checked;
    }

    return (false, false);
}
