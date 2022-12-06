pub mod input;

pub fn day_6_part_1(input: &'static str) -> String {
    find_packet_marker(input, 4).to_string()
}

pub fn day_6_part_2(input: &'static str) -> String {
    find_packet_marker(input, 14).to_string()
}

fn find_packet_marker(input: &'static str, buffer_len: usize) -> usize {
    let mut buffer = String::new();

    let mut index: Option<usize> = None;
    for (idx, character) in input.chars().enumerate() {
        buffer.push(character);
        if buffer.len() > buffer_len {
            buffer.remove(0);
        }

        // Don't start checking until the buffer is at least four characters long
        if buffer.len() < buffer_len {
            continue;
        }

        let mut done = true;
        for buffer_char in buffer.chars() {
            if buffer.matches(buffer_char).count() > 1 {
                done = false;
            }
        }
        if done == true {
            index = Some(idx + 1);
            break;
        }
    }

    index.expect("Did not find a match")
}

#[cfg(test)]
mod test {
    use super::*;

    // static TEST_INPUT: &str = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#;

    #[test]
    fn test_find_packet_marker_4() {
        let packet: usize = find_packet_marker(r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, 4);
        assert_eq!(packet, 5);

        let packet: usize = find_packet_marker(r#"nppdvjthqldpwncqszvftbrmjlhg"#, 4);
        assert_eq!(packet, 6);

        let packet: usize = find_packet_marker(r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#, 4);
        assert_eq!(packet, 10);

        let packet: usize = find_packet_marker(r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#, 4);
        assert_eq!(packet, 11);
    }

    #[test]
    fn test_find_packet_marker_14() {
        let packet: usize = find_packet_marker(r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#, 14);
        assert_eq!(packet, 19);

        let packet: usize = find_packet_marker(r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, 14);
        assert_eq!(packet, 23);

        let packet: usize = find_packet_marker(r#"nppdvjthqldpwncqszvftbrmjlhg"#, 14);
        assert_eq!(packet, 23);

        let packet: usize = find_packet_marker(r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#, 14);
        assert_eq!(packet, 29);

        let packet: usize = find_packet_marker(r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#, 14);
        assert_eq!(packet, 26);
    }

    #[test]
    fn part_1() {}

    #[test]
    fn part_2() {}
}
