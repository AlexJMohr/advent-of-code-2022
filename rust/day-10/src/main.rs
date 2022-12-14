use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{self, line_ending},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};
use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Inst {
    Noop,
    Addx(i32),
}

fn noop(input: &str) -> IResult<&str, Inst> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Inst::Noop))
}

fn addx(input: &str) -> IResult<&str, Inst> {
    let (input, amount) = preceded(tag("addx "), complete::i32)(input)?;
    Ok((input, Inst::Addx(amount)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Inst>> {
    separated_list0(line_ending, alt((noop, addx)))(input)
}

fn part1(input: &str) -> i32 {
    let (_, instructions) = instructions(input).unwrap();

    let mut signal_strength = 0;
    let mut value = 1;
    let mut pipeline = VecDeque::from([Inst::Noop]);

    let mut cycle = 0;
    for instruction in instructions.iter() {
        cycle += 1;

        match instruction {
            Inst::Addx(_) => {
                pipeline.push_back(Inst::Noop);
                pipeline.push_back(*instruction);
            }
            Inst::Noop => pipeline.push_back(*instruction),
        }

        match pipeline.pop_front().unwrap() {
            Inst::Addx(val) => value += val,
            _ => (),
        }
        if (cycle - 20) % 40 == 0 {
            let current_signal_strength = cycle * value;
            signal_strength += current_signal_strength;
        }
    }

    // drain the pipeline
    while let Some(inst) = pipeline.pop_front() {
        cycle += 1;
        match inst {
            Inst::Addx(val) => value += val,
            _ => (),
        }
        if (cycle - 20) % 40 == 0 {
            let current_signal_strength = cycle * value;
            signal_strength += current_signal_strength;
        }
    }

    signal_strength
}

fn part2(input: &str) -> String {
    let (_, instructions) = instructions(input).unwrap();

    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut display = vec![vec!['.'; WIDTH]; HEIGHT];

    let mut sprite_x: i32 = 1;

    let mut pipeline = VecDeque::new();

    let mut cycle = 0;
    for instruction in instructions.iter() {
        pipeline.push_back(Inst::Noop);
        if let Inst::Addx(x) = instruction {
            pipeline.push_back(Inst::Addx(*x));
        }
        let x = cycle % WIDTH;
        let y = (cycle / WIDTH) % HEIGHT;
        if ((sprite_x - 1)..=(sprite_x + 1)).contains(&(x as i32)) {
            display[y][x] = '#';
        }

        if let Inst::Addx(x) = pipeline.pop_front().unwrap() {
            sprite_x += x;
        }
        cycle += 1;
    }

    // drain the pipeline
    while let Some(inst) = pipeline.pop_front() {
        let x = cycle % WIDTH;
        let y = (cycle / WIDTH) % HEIGHT;
        if ((sprite_x - 1)..=(sprite_x + 1)).contains(&(x as i32)) {
            display[y][x] = '#';
        }
        cycle += 1;
        if let Inst::Addx(x) = inst {
            sprite_x += x;
        }
    }

    let mut res = String::new();
    for row in display.iter() {
        for c in row.iter() {
            res.push(*c);
        }
        res.push('\n');
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_works() {
        assert_eq!(Ok(("", Inst::Noop)), noop("noop"));
    }

    #[test]
    fn addx_works() {
        assert_eq!(Ok(("", Inst::Addx(8))), addx("addx 8"));
        assert_eq!(Ok(("", Inst::Addx(-19))), addx("addx -19"));
    }

    #[test]
    fn instructions_works() {
        assert_eq!(
            Ok(("", vec![Inst::Addx(12), Inst::Noop])),
            instructions("addx 12\nnoop")
        )
    }
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        assert_eq!(13140, part1(INPUT));
    }

    #[test]
    fn part2_works() {
        let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(output, part2(INPUT));
    }
}
