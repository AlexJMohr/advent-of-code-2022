fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let grid = parse_grid(input);

    let size = grid[0].len(); // assuming the grid is square
    let mut visible_count = 0;
    for (row_number, row) in grid.iter().enumerate() {
        for (col_number, tree_height) in row.iter().enumerate() {
            if row_number == 0
                || row_number == size - 1
                || col_number == 0
                || col_number == size - 1
            {
                visible_count += 1;
                continue;
            }
            // left to right
            match grid[row_number][0..col_number].iter().max() {
                Some(max) => {
                    if max < tree_height {
                        visible_count += 1;
                        continue;
                    }
                }
                None => {
                    visible_count += 1;
                    continue;
                }
            };
            // top to bottom
            match grid[0..row_number].iter().map(|v| v[col_number]).max() {
                Some(max) => {
                    if max < *tree_height {
                        visible_count += 1;
                        continue;
                    }
                }
                None => {
                    visible_count += 1;
                    continue;
                }
            }
            // right to left
            match grid[row_number][col_number + 1..].iter().rev().max() {
                Some(max) => {
                    if max < tree_height {
                        visible_count += 1;
                        continue;
                    }
                }
                None => {
                    visible_count += 1;
                    continue;
                }
            }
            // bottom to top
            match grid[row_number + 1..]
                .iter()
                .rev()
                .map(|v| v[col_number])
                .max()
            {
                Some(max) => {
                    if max < *tree_height {
                        visible_count += 1;
                        continue;
                    }
                }
                None => {
                    visible_count += 1;
                    continue;
                }
            }
        }
    }
    visible_count
}

fn part2(input: &str) -> u32 {
    let grid = parse_grid(input);

    let size = grid[0].len();
    let mut best = 0;
    for r in 1..(size - 1) {
        for c in 1..(size - 1) {
            let current_tree = grid[r][c];

            let mut left = 0;
            for cc in (0..c).rev() {
                let tree = grid[r][cc];
                left += 1;
                if tree >= current_tree {
                    break;
                }
            }

            let mut up = 0;
            for rr in (0..r).rev() {
                let tree = grid[rr][c];
                up += 1;
                if tree >= current_tree {
                    break;
                }
            }

            let mut right = 0;
            for cc in (c + 1)..size {
                let tree = grid[r][cc];
                right += 1;
                if tree >= current_tree {
                    break;
                }
            }

            let mut down = 0;
            for rr in (r + 1)..size {
                let tree = grid[rr][c];
                down += 1;
                if tree >= current_tree {
                    break;
                }
            }

            let score = left * up * right * down;
            if score > best {
                best = score;
            }
        }
    }
    best as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        assert_eq!(21, part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!(8, part2(INPUT));
    }
}
