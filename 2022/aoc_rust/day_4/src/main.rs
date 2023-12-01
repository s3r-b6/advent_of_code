fn main() {
    let input = std::fs::read_to_string("D:\\Rust\\advent_of_code\\day_4\\src\\input.txt").unwrap();

    let overlap_count_part1 = input
        .lines()
        .filter(|line| count_overlaps_part1(line))
        .count();

    let overlap_count_part2 = input
        .lines()
        .filter(|line| count_overlaps_part2(line))
        .count();

    println!(
        "{} pairs have ranges that are contained",
        overlap_count_part1
    );
    println!(
        "{} pairs have assignments that overlap",
        overlap_count_part2
    );
}

fn count_overlaps_part1(line: &str) -> bool {
    let mut left_right: Vec<&str> = vec![];
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    //used tuples and split_at method first, but as it is inclusive had to replace '-' and ','
    //with '', so I found this a little cleaner. Should also be somewhat faster.
    line.split(",")
        .for_each(|line_half| left_right.push(line_half));
    left_right[0]
        .split("-")
        .for_each(|range_half| left.push(range_half.parse().unwrap()));
    left_right[1]
        .split("-")
        .for_each(|range_half| right.push(range_half.parse().unwrap()));

    let first_range: (u32, u32);
    let second_range: (u32, u32);

    //checks which range starts at a lower num, if they start on the same num, check if the biggest
    //number of the range is different, the bigger is the range that could contain the other
    if left[0] == right[0] && left[1] > right[1] || left[0] < right[0] {
        first_range = (left[0], left[1]);
        second_range = (right[0], right[1]);
    } else {
        first_range = (right[0], right[1]);
        second_range = (left[0], left[1]);
    }

    if first_range.0 <= second_range.0 && first_range.1 >= second_range.1 {
        return true;
    } else {
        return false;
    }
}

fn count_overlaps_part2(line: &str) -> bool {
    let mut left_right: Vec<&str> = vec![];
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    //used tuples and split_at method first, but as it is inclusive had to replace '-' and ','
    //with '', so I found this a little cleaner. Should also be somewhat faster.
    line.split(",")
        .for_each(|line_half| left_right.push(line_half));
    left_right[0]
        .split("-")
        .for_each(|range_half| left.push(range_half.parse().unwrap()));
    left_right[1]
        .split("-")
        .for_each(|range_half| right.push(range_half.parse().unwrap()));

    let first_range: (u32, u32);
    let second_range: (u32, u32);

    //checks which range starts at a lower num, if they start on the same num, check if the biggest
    //number of the range is different, the bigger is the range that could contain the other
    if left[0] == right[0] && left[1] > right[1] || left[0] < right[0] {
        first_range = (left[0], left[1]);
        second_range = (right[0], right[1]);
    } else {
        first_range = (right[0], right[1]);
        second_range = (left[0], left[1]);
    }

    //if second's range lower is inside the range (it is <= than the first's higher)
    if second_range.0 <= first_range.1 {
        return true;
    } else {
        return false;
    }
}
