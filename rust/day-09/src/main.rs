use std::collections::BTreeSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = BTreeSet::from([tail]);

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();

        for _ in 0..count {
            head = match dir {
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                "R" => (head.0 + 1, head.1),
                "L" => (head.0 - 1, head.1),
                _ => unreachable!(),
            };

            let h: i32 = head.0 - tail.0;
            let v: i32 = head.1 - tail.1;

            if h.abs() > 1 || v.abs() > 1 {
                if h == 0 && v.abs() > 1 {
                    let dy = if v > 1 { 1 } else { -1 };
                    tail = (tail.0, tail.1 + dy);
                } else if h.abs() > 1 && v == 0 {
                    let dx = if h > 1 { 1 } else { -1 };
                    tail = (tail.0 + dx, tail.1);
                } else {
                    let dx = if h > 0 { 1 } else { -1 };
                    let dy = if v > 0 { 1 } else { -1 };
                    tail = (tail.0 + dx, tail.1 + dy);
                }
            }

            tail_positions.insert(tail);
        }
    }
    tail_positions.len()
}

fn part2(input: &str) -> usize {
    let mut knots = Vec::new();
    for _ in 0..10 {
        knots.push((0, 0));
    }
    let mut tail_positions = BTreeSet::from([(0, 0)]);

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();

        for _ in 0..count {
            for i in 1..knots.len() {
                let mut parent = knots[i - 1];
                let mut current = knots[i];
                if i == 1 {
                    parent = match dir {
                        "U" => (parent.0, parent.1 + 1),
                        "D" => (parent.0, parent.1 - 1),
                        "R" => (parent.0 + 1, parent.1),
                        "L" => (parent.0 - 1, parent.1),
                        _ => unreachable!(),
                    };
                    knots[i - 1] = parent;
                }

                let h: i32 = parent.0 - current.0;
                let v: i32 = parent.1 - current.1;

                if h.abs() > 1 || v.abs() > 1 {
                    if h == 0 && v.abs() > 1 {
                        let dy = if v > 1 { 1 } else { -1 };
                        current = (current.0, current.1 + dy);
                    } else if h.abs() > 1 && v == 0 {
                        let dx = if h > 1 { 1 } else { -1 };
                        current = (current.0 + dx, current.1);
                    } else {
                        let dx = if h > 0 { 1 } else { -1 };
                        let dy = if v > 0 { 1 } else { -1 };
                        current = (current.0 + dx, current.1 + dy);
                    }
                }

                knots[i] = current;

                if i == 9 {
                    tail_positions.insert(current);
                }
            }
        }
    }
    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, part1(input));
    }

    #[test]
    fn part2_works() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, part2(input));
    }
}
