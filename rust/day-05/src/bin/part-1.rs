use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", day_05::part1(&input));
}
