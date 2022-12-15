use std::collections::{BTreeSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input).unwrap());
}

fn parse_heightmap(input: &str) -> (Vec<Vec<i32>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    let heightmap = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, c)| match c {
                    b'a'..=b'z' => c - b'a',
                    b'S' => {
                        start = Some((x, y));
                        0
                    }
                    b'E' => {
                        end = Some((x, y));
                        b'z' - b'a'
                    }
                    _ => unreachable!(),
                } as i32)
                .collect::<Vec<_>>()
        })
        .collect();
    (heightmap, start.unwrap(), end.unwrap())
}

fn part1(input: &str) -> Option<usize> {
    let (heightmap, start, end) = parse_heightmap(input);
    let height = heightmap.len();
    let width = heightmap[0].len();
    let mut queue = VecDeque::<_>::from([vec![start]]);
    let mut visited = BTreeSet::new();

    while let Some(path) = queue.pop_front() {
        let pos = path[path.len() - 1];
        if pos == end {
            return Some(path.len() - 1);
        }
        if visited.contains(&pos) {
            continue;
        }

        let start_y = if pos.1 > 0 { pos.1 - 1 } else { pos.1 };
        let end_y = if pos.1 < height - 1 { pos.1 + 1 } else { pos.1 };
        for y in start_y..=end_y {
            let new_pos = (pos.0, y);
            if new_pos == pos
                || visited.contains(&new_pos)
                || heightmap[new_pos.1][new_pos.0] - heightmap[pos.1][pos.0] > 1
            {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(new_pos);
            queue.push_back(new_path);
        }

        let start_x = if pos.0 > 0 { pos.0 - 1 } else { pos.0 };
        let end_x = if pos.0 < width - 1 { pos.0 + 1 } else { pos.0 };
        for x in start_x..=end_x {
            let new_pos = (x, pos.1);
            if new_pos == pos
                || visited.contains(&new_pos)
                || heightmap[new_pos.1][new_pos.0] - heightmap[pos.1][pos.0] > 1
            {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(new_pos);
            queue.push_back(new_path);
        }
        visited.insert(pos);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn parse_heightmap_works() {
        let heightmap = vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ];
        let start = (0, 0);
        let end = (5, 2);
        assert_eq!((heightmap, start, end), parse_heightmap(&INPUT));
    }

    #[test]
    fn part1_works() {
        assert_eq!(Some(31), part1(INPUT));
    }
}
