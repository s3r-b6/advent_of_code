use std::collections::HashMap;

/*


*/
fn main() {
    let contents = include_str!("../input.txt");

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

    let mut curr_pos = "AAA";
    let mut count = 0;

    while curr_pos != "ZZZ" {
        for ch in instr.chars() {
            curr_pos = if ch == 'L' {
                maps.get(curr_pos).unwrap().0
            } else {
                maps.get(curr_pos).unwrap().1
            };

            count += 1;
        }
    }

    println!("{}, {}", count, curr_pos);
}
