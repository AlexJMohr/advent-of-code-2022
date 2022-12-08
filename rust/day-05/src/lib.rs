use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, multispace1, newline, u32 as u32_},
    multi::{many1, separated_list0},
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq)]
struct Crate(char);

fn krate(input: &str) -> IResult<&str, Option<Crate>> {
    let (input, krate) = delimited(tag("["), anychar, tag("]"))(input)?;
    Ok((input, Some(Crate(krate))))
}

fn empty_slot(input: &str) -> IResult<&str, Option<Crate>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

fn crate_slot(input: &str) -> IResult<&str, Option<Crate>> {
    let (input, krate) = alt((krate, empty_slot))(input)?;
    Ok((input, krate))
}

fn crate_row(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (input, crates) = separated_list0(tag(" "), crate_slot)(input)?;
    Ok((input, crates))
}

fn crate_stacks(input: &str) -> IResult<&str, Vec<Vec<Option<Crate>>>> {
    let (input, crate_stacks) = separated_list0(newline, crate_row)(input)?;
    Ok((input, crate_stacks))
}

#[derive(Debug, PartialEq)]
struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

fn mov(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = u32_(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = u32_(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = u32_(input)?;
    Ok((
        input,
        Move {
            amount,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, moves) = separated_list0(newline, mov)(input)?;
    Ok((input, moves))
}

fn numbers_row(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_until("\n")(input)?;
    Ok((input, ()))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<Option<Crate>>>, Vec<Move>)> {
    let (input, stacks) = crate_stacks(input)?;
    // eat the numbers row
    let (input, _) = numbers_row(input)?;
    // eat the blank line
    let (input, _) = many1(multispace1)(input)?;
    let (input, moves) = moves(input)?;
    Ok((input, (stacks, moves)))
}

pub fn part1(input: &str) -> String {
    let (_, (stacks_horiz, moves)) = parse_input(input).unwrap();

    let mut stacks: Vec<Vec<&Crate>> = vec![];
    for _ in 0..stacks_horiz[0].len() {
        stacks.push(vec![]);
    }

    for row in stacks_horiz.iter().rev() {
        for (i, maybe_crate) in row.iter().enumerate() {
            if let Some(krate) = maybe_crate {
                stacks[i].push(krate);
            }
        }
    }

    for mov in moves.iter() {
        for _ in 0..mov.amount {
            let krate = stacks[mov.from as usize].pop().unwrap();
            stacks[mov.to as usize].push(krate);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .map(|Crate(char)| char)
        .collect()
}

pub fn part2(input: &str) -> String {
    let (_, (stacks_horiz, moves)) = parse_input(input).unwrap();

    let mut stacks: Vec<Vec<&Crate>> = vec![];
    for _ in 0..stacks_horiz[0].len() {
        stacks.push(vec![]);
    }

    for row in stacks_horiz.iter().rev() {
        for (i, maybe_crate) in row.iter().enumerate() {
            if let Some(krate) = maybe_crate {
                stacks[i].push(krate);
            }
        }
    }

    for Move { amount, from, to } in moves.iter() {
        let len = stacks[*from as usize].len();
        let crates = stacks[*from as usize]
            .drain((len - *amount as usize)..)
            .collect::<Vec<&Crate>>();
        for krate in crates {
            stacks[*to as usize].push(krate);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .map(|Crate(char)| char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ".to_string());
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD".to_string());
    }

    #[test]
    fn krate_works() {
        assert_eq!(Ok(("", Some(Crate('A')))), krate("[A]"))
    }

    #[test]
    fn empty_slot_works() {
        assert_eq!(Ok(("", None)), empty_slot("   "))
    }

    #[test]
    fn crate_slot_works() {
        assert_eq!(Ok(("", Some(Crate('M')))), crate_slot("[M]"));
        assert_eq!(Ok(("", None)), crate_slot("   "));
    }

    #[test]
    fn crate_row_works() {
        assert_eq!(
            Ok(("", vec![Some(Crate('A')), None, Some(Crate('C'))])),
            crate_row("[A]     [C]")
        )
    }

    #[test]
    fn crate_stacks_works() {
        assert_eq!(
            Ok((
                "",
                vec![
                    vec![None, Some(Crate('Z')), Some(Crate('V'))],
                    vec![Some(Crate('A')), Some(Crate('B')), None],
                ]
            )),
            crate_stacks("    [Z] [V]\n[A] [B]    ")
        )
    }

    #[test]
    fn mov_works() {
        assert_eq!(
            Ok((
                "",
                Move {
                    amount: 3,
                    from: 8,
                    to: 0
                }
            )),
            mov("move 3 from 9 to 1")
        )
    }

    #[test]
    fn moves_works() {
        assert_eq!(
            Ok((
                "",
                vec![
                    Move {
                        amount: 1,
                        from: 2,
                        to: 3
                    },
                    Move {
                        amount: 3,
                        from: 0,
                        to: 5
                    }
                ]
            )),
            moves("move 1 from 3 to 4\nmove 3 from 1 to 6")
        )
    }
}
