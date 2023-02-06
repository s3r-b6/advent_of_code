use core::panic;
// ^= is similar to +=
//xor against a bit in the bit that represents that char in the i32

pub fn find_marker_position_bin(input: &Vec<u8>, size: usize) -> usize {
    let pos = input.windows(size).position(|window| {
        let mut unique_chars = 0u32;
        for char_value in window {
            unique_chars ^= 1 << char_value % 32;
        }
        unique_chars.count_ones() == size as u32
    });

    return pos.unwrap_or_else(|| panic!("Not found")) + size;
}
