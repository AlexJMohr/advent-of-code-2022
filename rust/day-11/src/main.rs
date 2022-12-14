use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::line_ending,
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    divisor: u32,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
    inspect_count: u32,
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = nom::character::complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, items) = starting_items(input)?;
    let (input, _) = line_ending(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = line_ending(input)?;
    let (input, divisor) = divisor(input)?;
    let (input, _) = line_ending(input)?;
    let (input, true_monkey_idx) = true_monkey_idx(input)?;
    let (input, _) = line_ending(input)?;
    let (input, false_monkey_idx) = false_monkey_idx(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            divisor,
            true_monkey_idx,
            false_monkey_idx,
            inspect_count: 0,
        },
    ))
}

fn starting_items(input: &str) -> IResult<&str, VecDeque<u32>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), nom::character::complete::u32)(input)?;
    Ok((input, VecDeque::from(items)))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = old ")(input)?;
    let (input, op) = alt((tag("+"), tag("*")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, val) = alt((take_while1(|c: char| c.is_digit(10)), tag("old")))(input)?;

    let op = match (op, val) {
        ("*", "old") => Operation::Square,
        ("*", v) => Operation::Mul(v.parse().unwrap()),
        ("+", v) => Operation::Add(v.parse().unwrap()),
        _ => unreachable!(),
    };

    // let (input, val) = nom::character::complete::u32(input)?;
    // let op = match op {
    //     "*" => Operation::Mul(val),
    //     "+" => Operation::Add(val),
    //     "old" => Operation::Square,
    //     _ => unreachable!(),
    // };
    Ok((input, op))
}

fn divisor(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    nom::character::complete::u32(input)
}

fn true_monkey_idx(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, idx) = nom::character::complete::u64(input)?;
    Ok((input, idx as usize))
}

fn false_monkey_idx(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("    If false: throw to monkey ")(input)?;
    let (input, idx) = nom::character::complete::u64(input)?;
    Ok((input, idx as usize))
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(tag("\n\n"), monkey)(input)?;
    dbg!(input);
    Ok((input, monkeys))
}

fn part1(input: &str) -> u32 {
    let (_, mut monkeys) = monkeys(input).unwrap();
    println!("{:#?}", monkeys);
    for round in 0..2 {
        // FIXME: find a better way than cloning
        for (i, monkey) in monkeys.clone().iter_mut().enumerate() {
            println!("Monkey {}:", i);
            // FIXME: the monkey should consider the items it receives this round
            while let Some(item) = monkey.items.pop_front() {
                println!("  Monkey inspects item with a worry level of {}.", item);
                monkeys[i].inspect_count += 1;

                let item = match monkey.operation {
                    Operation::Add(val) => {
                        let new_val = item + val;
                        println!("    Worry level increases by {} to {}", val, new_val);
                        new_val
                    }
                    Operation::Mul(val) => {
                        let new_val = item * val;
                        println!("    Worry level is multiplied by {} to {}.", val, new_val);
                        new_val
                    }
                    Operation::Square => {
                        let new_val = item * item;
                        println!("    Worry level is multiplied by {} to {}", item, item);
                        new_val
                    }
                };

                let item = item / 3;
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                    item
                );

                let idx = if item % monkey.divisor == 0 {
                    println!(
                        "    Current worry level is divisible by {}.",
                        monkey.divisor
                    );
                    monkey.true_monkey_idx
                } else {
                    println!(
                        "    Current worry level is not divisible by {}.",
                        monkey.divisor
                    );
                    monkey.false_monkey_idx
                };

                monkeys[idx].items.push_back(item);
                println!(
                    "    Item with worry level {} is thrown to monkey {}.",
                    item, idx
                );
            }
            println!("  Inspected {} items in total", monkeys[i].inspect_count);
        }
        println!("******** ROUND {} END **********", round);
    }
    let mut counts = monkeys
        .iter()
        .map(|monkey| monkey.inspect_count)
        .collect::<Vec<u32>>();
    counts.sort();
    println!("{:?}", counts);
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn starting_items_works() {
        assert_eq!(
            Ok(("", VecDeque::from([79, 98]))),
            starting_items("  Starting items: 79, 98")
        );
    }

    #[test]
    fn operation_works() {
        assert_eq!(
            Ok(("", Operation::Add(8))),
            operation("  Operation: new = old + 8")
        );
        assert_eq!(
            Ok(("", Operation::Mul(11))),
            operation("  Operation: new = old * 11")
        );
        assert_eq!(
            Ok(("", Operation::Square)),
            operation("  Operation: new = old * old")
        );
    }

    #[test]
    fn divisor_works() {
        assert_eq!(Ok(("", 13)), divisor("  Test: divisible by 13"))
    }

    #[test]
    fn true_monkey_idx_works() {
        assert_eq!(
            Ok(("", 2)),
            true_monkey_idx("    If true: throw to monkey 2")
        );
    }

    #[test]
    fn false_monkey_idx_works() {
        assert_eq!(
            Ok(("", 4)),
            false_monkey_idx("    If false: throw to monkey 4")
        );
    }

    #[test]
    fn monkey_works() {
        assert_eq!(
            Ok((
                "",
                Monkey {
                    items: VecDeque::from([1, 2, 3]),
                    operation: Operation::Add(3),
                    divisor: 8,
                    true_monkey_idx: 3,
                    false_monkey_idx: 2,
                    inspect_count: 0,
                }
            )),
            monkey(
                "Monkey 0:
  Starting items: 1, 2, 3
  Operation: new = old + 3
  Test: divisible by 8
    If true: throw to monkey 3
    If false: throw to monkey 2"
            )
        )
    }

    #[test]
    fn monkeys_works() {
        assert_eq!(
            Ok((
                "",
                vec![
                    Monkey {
                        items: VecDeque::from([34, 12]),
                        operation: Operation::Mul(5),
                        divisor: 2,
                        true_monkey_idx: 2,
                        false_monkey_idx: 1,
                        inspect_count: 0,
                    },
                    Monkey {
                        items: VecDeque::from([9]),
                        operation: Operation::Add(11),
                        divisor: 4,
                        true_monkey_idx: 1,
                        false_monkey_idx: 0,
                        inspect_count: 0,
                    },
                ]
            )),
            monkeys(
                "Monkey 0:
  Starting items: 34, 12
  Operation: new = old * 5
  Test: divisible by 2
    If true: throw to monkey 2
    If false: throw to monkey 1

Monkey 1:
  Starting items: 9
  Operation: new = old + 11
  Test: divisible by 4
    If true: throw to monkey 1
    If false: throw to monkey 0"
            )
        )
    }

    #[test]
    fn monkeys_works_on_input() {
        assert_eq!(
            Ok((
                "",
                vec![
                    Monkey {
                        items: VecDeque::from([79, 98]),
                        operation: Operation::Mul(19),
                        divisor: 23,
                        true_monkey_idx: 2,
                        false_monkey_idx: 3,
                        inspect_count: 0,
                    },
                    Monkey {
                        items: VecDeque::from([54, 65, 75, 74]),
                        operation: Operation::Add(6),
                        divisor: 19,
                        true_monkey_idx: 2,
                        false_monkey_idx: 0,
                        inspect_count: 0,
                    },
                    Monkey {
                        items: VecDeque::from([79, 60, 97]),
                        operation: Operation::Square,
                        divisor: 13,
                        true_monkey_idx: 1,
                        false_monkey_idx: 3,
                        inspect_count: 0,
                    },
                    Monkey {
                        items: VecDeque::from([74]),
                        operation: Operation::Add(3),
                        divisor: 17,
                        true_monkey_idx: 0,
                        false_monkey_idx: 1,
                        inspect_count: 0,
                    },
                ]
            )),
            monkeys(INPUT)
        )
    }

    #[test]
    fn part1_works() {
        assert_eq!(10605, part1(INPUT));
    }
}
