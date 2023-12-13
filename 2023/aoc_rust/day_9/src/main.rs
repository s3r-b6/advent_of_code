fn main() {
    let contents = include_str!("../input.txt");

    let l_values: Vec<Vec<i32>> = contents
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|part| part.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut answer = (0, 0);
    for l in l_values {
        let res = get_extrapolated_value_p1(l);
        answer.0 += res.0;
        answer.1 += res.1;
    }

    println!("P1: {}", answer.0);
    println!("P2: {}", answer.1);
}

fn get_extrapolated_value_p1(parts: Vec<i32>) -> (i32, i32) {
    let mut diffs: Vec<Vec<i32>> = vec![];
    let mut parts = parts;

    let first_num = parts[0];
    let last_num = parts[parts.len() - 1];
    loop {
        let mut new_parts = vec![];

        let mut prev: Option<i32> = None;
        for part in parts {
            if let Some(p) = prev {
                new_parts.push(part - p);
                prev = Some(part);
            } else {
                prev = Some(part);
            }
        }

        if new_parts.iter().all(|p| *p == 0) {
            break; // Do not store the 0 vec
        }

        diffs.push(new_parts.clone());
        parts = new_parts;
    }

    (part1(&diffs, last_num), part_2(&diffs, first_num))
}

fn part1(diffs: &Vec<Vec<i32>>, last_num: i32) -> i32 {
    let mut acc = 0;
    let len = diffs.len() - 1;
    for i in 0..=len {
        acc += diffs[len - i][diffs[len - i].len() - 1];
    }

    acc + last_num
}

fn part_2(diffs: &Vec<Vec<i32>>, first_num: i32) -> i32 {
    let mut acc = 0;
    let len = diffs.len() - 1;
    for i in 0..=len {
        acc = diffs[len - i][0] - acc;
    }

    first_num - acc
}
