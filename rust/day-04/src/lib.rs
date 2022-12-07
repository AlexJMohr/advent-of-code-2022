use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list0, sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
struct RangePair(RangeInclusive<u32>, RangeInclusive<u32>);

fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
    Ok((input, start..=end))
}

fn range_pair(input: &str) -> IResult<&str, RangePair> {
    let (input, (range_a, range_b)) = separated_pair(range, tag(","), range)(input)?;
    Ok((input, RangePair(range_a, range_b)))
}

fn section_assignments(input: &str) -> IResult<&str, Vec<RangePair>> {
    let (input, assignments) = separated_list0(complete::newline, range_pair)(input)?;
    Ok((input, assignments))
}

pub fn part1(input: &str) -> usize {
    let (_, assignments) = section_assignments(input).unwrap();
    assignments
        .iter()
        .filter(|RangePair(range_a, range_b)| {
            range_a.clone().into_iter().all(|a| range_b.contains(&a))
                || range_b.clone().into_iter().all(|b| range_a.contains(&b))
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let (_, assignments) = section_assignments(input).unwrap();
    assignments
        .iter()
        .filter(|RangePair(range_a, range_b)| {
            range_a.clone().into_iter().any(|a| range_b.contains(&a))
                || range_b.clone().into_iter().any(|b| range_a.contains(&b))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_parse_range() {
        assert_eq!(Ok(("", 3..=10)), range("3-10"));
    }

    #[test]
    fn test_parse_range_pair() {
        assert_eq!(Ok(("", RangePair(1..=10, 4..=8))), range_pair("1-10,4-8"))
    }

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 4);
    }
}
