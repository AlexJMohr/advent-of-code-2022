use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Command<'a> {
    Cd(Cd<'a>),
    ListDir(Vec<Node<'a>>),
}

#[derive(Debug, PartialEq)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug, PartialEq)]
enum Node<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Node> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        take_while1(|c: char| c.is_alphabetic() || c == '.'),
    )(input)?;
    Ok((input, Node::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Node::Dir(name)))
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag("/"), tag(".."), alpha1))(input)?;
    let cmd = match dir {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        _ => Command::Cd(Cd::Down(dir)),
    };
    Ok((input, cmd))
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, files) = separated_list1(line_ending, alt((file, directory)))(input)?;
    Ok((input, Command::ListDir(files)))
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, alt((ls, cd)))(input)
}

fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    command: &'a Command,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        Command::Cd(Cd::Root) => {
            context.push("/");
        }
        Command::Cd(Cd::Up) => {
            context.pop();
        }
        Command::Cd(Cd::Down(name)) => {
            context.push(name);
        }
        Command::ListDir(files) => {
            let sum = files
                .iter()
                .filter_map(|file| {
                    if let Node::File { size, .. } = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>();
            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            }
        }
    };
    (context, sizes)
}

fn part1(input: &str) -> u32 {
    let (_, cmds) = commands(input).unwrap();
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);
    sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum()
}

fn part2(input: &str) -> u32 {
    let (_, cmds) = commands(input).unwrap();
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    let total_size = 70_000_000;
    let needed_space = 30_000_000;
    let used_space = sizes.get(&vec!["/"]).unwrap();
    let current_free_space = total_size - used_space;
    let need_to_free = needed_space - current_free_space;

    let mut dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();
    dirs.sort();
    **dirs.iter().next().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn file_works() {
        assert_eq!(
            Ok((
                "",
                Node::File {
                    size: 123,
                    name: "myfile"
                }
            )),
            file("123 myfile")
        );
        assert_eq!(
            Ok((
                "",
                Node::File {
                    size: 7171717,
                    name: "another.exe"
                }
            )),
            file("7171717 another.exe")
        );
    }

    #[test]
    fn directory_works() {
        assert_eq!(Ok(("", Node::Dir("somedir"))), directory("dir somedir"));
    }

    #[test]
    fn cd_works() {
        assert_eq!(Ok(("", Command::Cd(Cd::Root))), cd("$ cd /"));
        assert_eq!(Ok(("", Command::Cd(Cd::Up))), cd("$ cd .."));
        assert_eq!(Ok(("", Command::Cd(Cd::Down("foo")))), cd("$ cd foo"));
    }

    #[test]
    fn ls_works() {
        assert_eq!(
            Ok(("", Command::ListDir(vec![Node::Dir("subdir")]))),
            ls("$ ls\ndir subdir")
        );
        assert_eq!(
            Ok((
                "",
                Command::ListDir(vec![Node::File {
                    size: 91,
                    name: "f.txt"
                }])
            )),
            ls("$ ls\n91 f.txt")
        );
        assert_eq!(
            Ok((
                "",
                Command::ListDir(vec![
                    Node::File {
                        size: 91,
                        name: "f.txt"
                    },
                    Node::Dir("hello")
                ])
            )),
            ls("$ ls\n91 f.txt\ndir hello")
        );
    }

    #[test]
    fn commands_works() {
        assert_eq!(
            Ok((
                "",
                vec![
                    Command::Cd(Cd::Root),
                    Command::ListDir(vec![Node::File {
                        size: 1,
                        name: "tmp"
                    }])
                ]
            )),
            commands("$ cd /\n$ ls\n1 tmp")
        )
    }

    #[test]
    fn part1_works() {
        assert_eq!(95437, part1(&INPUT));
    }
}
