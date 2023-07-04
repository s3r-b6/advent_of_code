use core::panic;

pub fn find_marker_position(input: &String, marker_type: &str) -> i32 {
    let mut char_buffer: String = "".to_string();
    let mut char_pos = 1;
    for character in input.chars() {
        let buffer_length = {
            if marker_type == "start" {
                4
            } else if marker_type == "message" {
                14
            } else {
                panic!("Bad args");
            }
        };

        if char_buffer.len() < buffer_length {
            char_buffer.push(character);
        } else {
            char_buffer.push(character);
            char_buffer = char_buffer[1..char_buffer.len()].to_string();
        }

        let mut repeated = false;
        if char_buffer.len() == buffer_length {
            // "aabc" ::::::: ? "abc" contains "a" => yes => break
            // "abcd" ::::::: ? "bcd" contains "a" => no  => "cd" contains "b" => no etc
            let mut char_buffer_copy = char_buffer.clone();
            for i in 0..char_buffer.len() {
                char_buffer_copy = char_buffer_copy[1..char_buffer_copy.len()].to_string();
                let character_a = char_buffer.chars().nth(i).unwrap();
                if char_buffer_copy.contains(character_a) {
                    repeated = true;
                    break;
                };
            }

            if repeated == false {
                println!("\nNon repeating char buffer: {}", char_buffer);
                return char_pos;
            }
        }

        char_pos += 1;
    }
    panic!("There should be a marker somewhere!");
}
