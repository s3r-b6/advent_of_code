use std::collections::HashMap;

fn main() {
    let contents = include_str!("../input.txt");
    //    let contents = "LR
    //
    //11A = (11B, XXX)
    //11B = (XXX, 11Z)
    //11Z = (11B, XXX)
    //22A = (22B, XXX)
    //22B = (22C, 22C)
    //22C = (22Z, 22Z)
    //22Z = (22B, 22B)
    //XXX = (XXX, XXX)";

    let mut lines = contents.lines();

    let instr = lines.next().unwrap();
    lines.next(); // Blank line separating
    let maps: HashMap<&str, (&str, &str)> = lines
        .map(|el| {
            let mut parts_1 = el.split('=');
            let idx = parts_1.next().unwrap().trim();
            let mut parts_2 = parts_1.next().unwrap().split(',');

            let map_l = parts_2.next().unwrap()[2..].trim();
            let map_r = parts_2.next().unwrap()[..4].trim();

            //println!("{}: L{}R{}", idx, map_l, map_r);
            (idx, (map_l, map_r))
        })
        .collect();

    println!("{}", part_1(instr, &maps));
    println!("{}", part_2(instr, &maps));
}

fn part_1(instr: &str, maps: &HashMap<&str, (&str, &str)>) -> usize {
    let directions: Vec<char> = instr.chars().collect();
    let mut curr_pos = "AAA";
    let mut count = 0;

    while curr_pos != "ZZZ" {
        let ch = directions[count % 2];

        curr_pos = if ch == 'L' {
            maps.get(curr_pos).unwrap().0
        } else {
            maps.get(curr_pos).unwrap().1
        };

        count += 1;
    }

    count
}

fn part_2(instr: &str, maps: &HashMap<&str, (&str, &str)>) -> usize {
    let directions: Vec<char> = instr.chars().collect();
    let mut positions: Vec<&&str> = maps.keys().filter(|k| k.ends_with('A')).collect();

    let mut count = 0;
    loop {
        for idx in 0..positions.len() {
            positions[idx] = if directions[count % 2] == 'L' {
                &maps.get(positions[idx]).unwrap().0
            } else {
                &maps.get(positions[idx]).unwrap().1
            };
        }

        count += 1;

        if positions.iter().all(|st| st.ends_with('Z')) {
            return count;
        }
    }
}
