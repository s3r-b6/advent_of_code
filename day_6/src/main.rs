use std::time::Instant;
mod answer_binary;
mod answer_old;
use answer_binary::find_marker_position_bin;
use answer_old::find_marker_position;

fn main() {
    let now = Instant::now();
    call_new_solution();
    println!("Completed in {:.2?}", now.elapsed());

    let now = Instant::now();
    call_old_solution();
    println!("Completed in {:.2?}", now.elapsed());
}

fn call_new_solution() {
    let input = std::fs::read("D:\\Rust\\advent_of_code\\day_6\\src\\input.txt").unwrap();
    let first_marker_pos_bin = find_marker_position_bin(&input, 4);
    println!("First packet at: {}", first_marker_pos_bin);
    let first_message_pos_bin = find_marker_position_bin(&input, 14);
    println!("First message at: {}", first_message_pos_bin);
}

fn call_old_solution() {
    let input = std::fs::read_to_string("D:\\Rust\\advent_of_code\\day_6\\src\\input.txt").unwrap();
    let first_marker_pos = find_marker_position(&input, "start");
    println!("First packet at: {}", first_marker_pos);
    let first_mssg_pos = find_marker_position(&input, "message");
    println!("First message at: {}", first_mssg_pos);
}
