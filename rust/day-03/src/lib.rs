#![feature(iter_array_chunks)]

use std::collections::BTreeSet;

fn letter_score(c: char) -> i32 {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .find_map(|(idx, ch)| if c == ch { Some(idx as i32 + 1) } else { None })
        .unwrap()
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(compartment_1, compartment_2)| {
            *compartment_1
                .chars()
                .collect::<BTreeSet<char>>()
                .intersection(&compartment_2.chars().collect())
                .next()
                .unwrap()
        })
        .map(|c| letter_score(c))
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|elf| elf.chars().collect::<BTreeSet<char>>())
        .array_chunks::<3>()
        .map(|chunk| {
            *(&(&chunk[0] & &chunk[1]) & &chunk[2])
                .iter()
                .next()
                .unwrap()
        })
        .map(|c| letter_score(c))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 70);
    }
}
