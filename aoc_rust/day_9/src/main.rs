fn main() {
    let input = std::fs::read_to_string("D:\\rust\\advent_of_code\\day_9\\src\\input.txt").unwrap();
    //head, tail || x , y
    let unique_positions_p1 = get_unique_pos_p1(&input);
    let unique_positions_p2 = get_unique_pos_p2(&input);
    println!(
        "P1: The tail has been to {} unique positions",
        unique_positions_p1
    );
    //~~~~ 2000
    println!(
        "P2: The tail has been to {} unique positions",
        unique_positions_p2
    );
}

fn get_unique_pos_p1(input: &String) -> usize {
    let mut already_visited: Vec<[i16; 2]> = vec![[0, 0]];
    let mut current_position = [[0, 0], [0, 0]];
    for line in input.lines() {
        let (dir, num) = line.split_once(" ").unwrap();
        let num: i8 = num.parse().unwrap();
        for _ in 0..num {
            let mut new_position = current_position[0];
            match dir {
                "U" => new_position[0] += 1,
                "L" => new_position[1] -= 1,
                "D" => new_position[0] -= 1,
                "R" => new_position[1] += 1,
                _ => panic!("should not happen"),
            }
            let x_diff = (new_position[0] - current_position[1][0] as i16).abs();
            let y_diff = (new_position[1] - current_position[1][1] as i16).abs();
            if !(new_position == current_position[1] || x_diff <= 1 && y_diff <= 1) {
                current_position[1] = current_position[0];
            }
            current_position[0] = new_position;
            if !already_visited.contains(&current_position[1]) {
                already_visited.push(current_position[1]);
            }
        }
    }
    already_visited.len()
}

//rope has now 10 knots H..9
//if H moves n positions, n knots are moved n-number of knots moved already) positions
//+ if a knot becomes disjointed, it does also move, so previous logic is the same more or less
fn get_unique_pos_p2(input: &String) -> usize {
    let mut already_visited: Vec<[i16; 2]> = vec![[0, 0]];
    //store the state of the whole rope

    let mut current_position = [[0, 0]; 10];
    for line in input.lines() {
        let (dir, num) = line.split_once(" ").unwrap();
        let num: i8 = num.parse().unwrap();
        let mut moved_knots = 0;
        for i in 0..9 {
            for j in 0..num - moved_knots {
                let mut new_position = current_position[i];
                if i == 0 {
                    match dir {
                        "U" => new_position[0] += 1,
                        "L" => new_position[1] -= 1,
                        "D" => new_position[0] -= 1,
                        "R" => new_position[1] += 1,
                        _ => panic!("should not happen"),
                    }
                }
                //if it is not the tail
                let x_diff = (new_position[0] - current_position[i + 1][0] as i16).abs();
                let y_diff = (new_position[1] - current_position[i + 1][1] as i16).abs();
                //see if the next knot has to follow bc the distance is too big (greater than 1,1 i.e., diagonally)
                if !(new_position == current_position[i + 1] || x_diff <= 1 && y_diff <= 1) {
                    current_position[i + 1] = current_position[i];
                }
                //always update the current knot
                current_position[i] = new_position;
                //if it is the tail, just update it
                if !already_visited.contains(&current_position[9]) {
                    already_visited.push(current_position[9]);
                }
            }

            moved_knots += 1;
        }
    }
    already_visited.len()
}
