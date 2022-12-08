use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    for (i, window) in input.as_bytes().windows(4).enumerate() {
        let set = window.iter().collect::<HashSet<&u8>>();
        if set.len() == 4 {
            return i + 4;
        }
    }
    0
}

pub fn part2(input: &str) -> usize {
    for (i, window) in input.as_bytes().windows(14).enumerate() {
        let set = window.iter().collect::<HashSet<&u8>>();
        if set.len() == 14 {
            return i + 14;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
}
