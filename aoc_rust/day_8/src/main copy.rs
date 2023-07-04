const GRID_SIZE: usize = 99;

fn main() {
    let input = std::fs::read_to_string("D:\\Rust\\advent_of_code\\day_8\\src\\input.txt").unwrap();

    //recreate the grid in memory
    let mut tree_grid: Vec<Vec<u8>> = vec![];
    for lines in input.lines() {
        let mut tree_row: Vec<u8> = vec![];
        for tree_height in lines.chars() {
            tree_row.push(tree_height.to_string().parse::<u8>().unwrap());
        }
        tree_grid.push(tree_row);
    }

    //count() says that both vecs are 99 length, so grid is 99x99
    let visible_trees = get_visible_trees(&tree_grid);

    println!("{} trees are visible from outside", visible_trees);

    println!("Biggest scenic score is: {}", get_scenic_score(&tree_grid));
}

//let's say current tree is tree_grid[1][1]
//    [0][1][2][3][4]
// [0] x  x  x  x  x
// [1] x  o  x  x  x
// [2] x  x  x  x  x
// [3] x  x  x  x  x
// [4] x  x  x  x  x
// tree[1][1] would only be seen if tree[0][1] && tree[1][0] are shorter
// tree [2][2] would be seen if trees[2][0] && trees[2][1] are shorter

//the question is how many individual trees are seen from outside, so if
//the number changes it should break

fn get_visible_trees(tree_grid: &Vec<Vec<u8>>) -> u16 {
    let mut visible_trees = 0;
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let prev_seen_trees = visible_trees;
            let current_tree = tree_grid[row][col];
            //the index of the last item is 98, not 99, as the first item is indexed 0
            if row == 0 || col == 0 || row == GRID_SIZE - 1 || col == GRID_SIZE - 1 {
                visible_trees += 1;
                continue;
            }

            //check above
            for upper_rows in 0..row {
                let tree_above = tree_grid[upper_rows][col];
                //if it is not the last item, check if higher or not
                if tree_above < current_tree && upper_rows == row - 1 {
                    //if the loop reaches the previous to current item and it is still lower, it is visible
                    visible_trees += 1;
                    break;
                } else if tree_above < current_tree {
                    continue;
                } else {
                    break;
                }
            }
            if prev_seen_trees != visible_trees {
                continue;
            }

            //check left
            for left_col in 0..col {
                let tree_in_left = tree_grid[row][left_col];
                if tree_in_left < current_tree && left_col == col - 1 {
                    visible_trees += 1;
                    break;
                } else if tree_in_left < current_tree {
                    continue;
                } else {
                    break;
                }
            }
            if prev_seen_trees != visible_trees {
                continue;
            }

            //check below
            for below_rows in row + 1..GRID_SIZE {
                let tree_below = tree_grid[below_rows][col];
                //here the check is from next item to the last one, but similar logic
                if below_rows == GRID_SIZE - 1 && tree_below < current_tree {
                    visible_trees += 1;
                    break;
                } else if tree_below < current_tree {
                    continue;
                } else {
                    break;
                }
            }
            if prev_seen_trees != visible_trees {
                continue;
            }

            //check right
            for right_col in col + 1..GRID_SIZE {
                let tree_in_right = tree_grid[row][right_col];
                if right_col == GRID_SIZE - 1 && tree_in_right < current_tree {
                    visible_trees += 1;
                    break;
                } else if tree_in_right < current_tree {
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    return visible_trees;
}

fn get_scenic_score(tree_grid: &Vec<Vec<u8>>) -> u16 {
    let mut scenic_scores: Vec<u16> = vec![];
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if row == 0 || col == 0 || row == GRID_SIZE - 1 || col == GRID_SIZE - 1 {
                continue;
            }

            let current_tree = tree_grid[row][col];

            let mut trees_seen_above = 0;
            let mut trees_seen_below = 0;
            let mut trees_seen_right = 0;
            let mut trees_seen_left = 0;
            let mut previous_tree = None;
            //check above
            for upper_rows in (0..row).rev() {
                trees_seen_above += 1;
                let tree_above = tree_grid[upper_rows][col];
                if previous_tree > Some(tree_above)
                    || (previous_tree == Some(tree_above) && previous_tree >= Some(current_tree))
                {
                    break;
                }
                previous_tree = Some(tree_above);
            }
            previous_tree = None;

            //check left
            for left_col in (0..col).rev() {
                trees_seen_left += 1;
                let tree_in_left = tree_grid[row][left_col];
                if previous_tree > Some(tree_in_left)
                    || (previous_tree == Some(tree_in_left) && previous_tree >= Some(current_tree))
                {
                    break;
                }
                previous_tree = Some(tree_in_left);
            }
            previous_tree = None;

            //check below
            for below_rows in row + 1..GRID_SIZE {
                let tree_below = tree_grid[below_rows][col];
                trees_seen_below += 1;
                if previous_tree > Some(tree_below)
                    || (previous_tree == Some(tree_below) && previous_tree >= Some(current_tree))
                {
                    break;
                }
                previous_tree = Some(tree_below);
            }
            previous_tree = None;

            //check right
            for right_col in col + 1..GRID_SIZE {
                trees_seen_right += 1;
                let tree_in_right = tree_grid[row][right_col];
                if previous_tree > Some(tree_in_right)
                    || (previous_tree == Some(tree_in_right) && previous_tree >= Some(current_tree))
                {
                    break;
                }
                previous_tree = Some(tree_in_right);
            }

            let scenic_score =
                trees_seen_above * trees_seen_below * trees_seen_left * trees_seen_right;
            scenic_scores.push(scenic_score);

            println!(
                "{} {} {} {}",
                trees_seen_above, trees_seen_below, trees_seen_left, trees_seen_right
            );
        }
    }

    scenic_scores.sort();
    return *scenic_scores.last().unwrap();
}
