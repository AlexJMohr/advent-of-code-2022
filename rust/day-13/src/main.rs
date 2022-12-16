use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::line_ending,
    multi::{many1, separated_list0},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (_, pairs) = pairs(input).unwrap();
    pairs
        .iter()
        .enumerate()
        .filter_map(
            |(idx, (left, right))| {
                if left <= right {
                    Some(idx + 1)
                } else {
                    None
                }
            },
        )
        .sum()
}

fn part2(input: &str) -> usize {
    let (_, mut packets) = packets(input).unwrap();
    let two = Item::List(vec![Item::List(vec![Item::Num(2)])]);
    let six = Item::List(vec![Item::List(vec![Item::Num(6)])]);
    packets.push(two.clone());
    packets.push(six.clone());
    packets.sort();
    let mut iter = packets
        .iter()
        .enumerate()
        .map(|(idx, packet)| (idx + 1, packet));
    let find = |(idx, item): (usize, &Item)| {
        if *item == two || *item == six {
            Some(idx)
        } else {
            None
        }
    };
    let first_idx = iter.find_map(find).unwrap();
    let second_idx = iter.find_map(find).unwrap();
    first_idx * second_idx
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Num(u32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Num(self_num), Item::Num(other_num)) => self_num.cmp(other_num),
            (Item::List(self_list), Item::List(other_list)) => self_list.cmp(other_list),
            (Item::Num(val), Item::List(other_list)) => vec![Item::Num(*val)].cmp(other_list),
            (Item::List(self_list), Item::Num(other_num)) => {
                self_list.cmp(&vec![Item::Num(*other_num)])
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn num(input: &str) -> IResult<&str, Item> {
    let (input, num) = nom::character::complete::u32(input)?;
    Ok((input, Item::Num(num)))
}

fn list(input: &str) -> IResult<&str, Item> {
    let (input, items) = delimited(
        tag("["),
        separated_list0(tag(","), alt((num, list))),
        tag("]"),
    )(input)?;
    Ok((input, Item::List(items)))
}

type Pair = (Item, Item);

fn list_pair(input: &str) -> IResult<&str, Pair> {
    let (input, lists) = separated_pair(list, line_ending, list)(input)?;
    Ok((input, lists))
}

// Pairs of packets, separated by 2 line endings
fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list0(pair(line_ending, line_ending), list_pair)(input)
}

/// List of packets ignoring line endings in between
fn packets(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list0(many1(line_ending), list)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        assert_eq!(13, part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!(140, part2(INPUT));
    }

    #[test]
    fn list_works() {
        assert_eq!(Ok(("", Item::List(vec![Item::Num(3)]))), list("[3]"));
        assert_eq!(
            Ok(("", Item::List(vec![Item::Num(3), Item::Num(9)]))),
            list("[3,9]")
        );
        assert_eq!(
            Ok((
                "",
                Item::List(vec![
                    Item::List(vec![Item::Num(42)]),
                    Item::Num(1),
                    Item::Num(2)
                ])
            )),
            list("[[42],1,2]")
        );
    }

    #[test]
    fn ord_works() {
        // 0 < 1
        assert!(Item::Num(0) < Item::Num(1));
        // 1 == 1
        assert!(Item::Num(1) == Item::Num(1));
        // 1 > 0
        assert!(Item::Num(1) > Item::Num(0));
        // [] < [1]
        assert!(Item::List(vec![]) < Item::List(vec![Item::Num(1)]));
        // [0] < [1]
        assert!(Item::List(vec![Item::Num(0)]) < Item::List(vec![Item::Num(1)]));
        // [1] == [1]
        assert!(Item::List(vec![Item::Num(1)]) == Item::List(vec![Item::Num(1)]));
        // [1] > []
        assert!(Item::List(vec![Item::Num(1)]) > Item::List(vec![]));
        // 1 > [0]
        assert!(Item::Num(1) > Item::List(vec![Item::Num(0)]));
        // 0 > []
        assert!(Item::Num(0) > Item::List(vec![]));
        // 0 < [1]
        assert!(Item::Num(0) < Item::List(vec![Item::Num(1)]));
        // [1,2,3] > [1,2]
        assert!(
            Item::List(vec![Item::Num(1), Item::Num(2), Item::Num(3)])
                > Item::List(vec![Item::Num(1), Item::Num(2)])
        );
        // [[[1]]] > [[[]]]
        assert!(
            Item::List(vec![Item::List(vec![Item::List(vec![Item::Num(1)])])])
                > Item::List(vec![Item::List(vec![Item::List(vec![])])])
        )
    }
}
