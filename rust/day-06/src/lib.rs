use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .position(|window| window.iter().collect::<HashSet<&u8>>().len() == 4)
        .unwrap()
        + 4
}

pub fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .position(|window| window.iter().collect::<HashSet<&u8>>().len() == 14)
        .unwrap()
        + 14
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
