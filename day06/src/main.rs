use std::collections::HashSet;

fn slice_is_unique<T: std::hash::Hash + std::cmp::Eq>(slice: &[T]) -> bool {
    return slice.iter().collect::<HashSet<_>>().len() == slice.len();
}

fn packet_start_position(input: &str, window_size: usize) -> Result<usize, ()> {
    let bytes = input.as_bytes();

    // Iterate over the string as a sliding window with 4 items.
    for (index, window) in bytes.windows(window_size).enumerate() {
        if slice_is_unique(window) {
            return Ok(index + window_size);
        }
    }

    return Err(());
}

fn main() {
    println!(
        "Part One: {}",
        packet_start_position(include_str!("../input.txt"), 4).unwrap(),
    );

    println!(
        "Part Two: {}",
        packet_start_position(include_str!("../input.txt"), 14).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_start_position() {
        assert_eq!(5, packet_start_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap());

        assert_eq!(19, packet_start_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap());
    }
}