extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::*,
    multi::separated_list1,
    IResult,
};
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Default, PartialEq)]
enum Type {
    #[default]
    File,
    Dir,
    CdCommand,
    LsCommand,
}

#[derive(Debug, PartialEq, Default)]
struct Entry {
    size: usize,
    name: String,
    entry: Type,
}

#[derive(Debug)]
struct FS {
    children: HashMap<String, Box<FS>>,
    files: HashMap<String, usize>,
    total_size: usize,
}

impl FS {
    fn insert_file(&mut self, path: &[String], name: String, size: usize) {
        if path.len() == 0 {
            self.files.insert(name, size);
        } else {
            if !self.children.contains_key(&path[0]) {
                self.children.insert(
                    path[0].clone(),
                    Box::new(FS {
                        children: HashMap::new(),
                        files: HashMap::new(),
                        total_size: 0,
                    }),
                );
            }
            let c = self.children.get_mut(&path[0]).unwrap();
            c.insert_file(&path[1..], name, size);
        }
        self.total_size += size;
    }

    fn sum_small(&mut self) -> usize {
        let mut total = 0;
        for (_, fs) in self.children.iter_mut() {
            total += fs.sum_small();
        }
        if self.total_size <= 100000 {
            total += self.total_size;
        }
        total
    }

    fn pick_closest_ge(&mut self, amount: usize) -> usize {
        let mut total = usize::MAX;
        for (_, fs) in self.children.iter_mut() {
            let candidate = fs.pick_closest_ge(amount);
            if candidate >= amount && candidate < total {
                total = candidate;
            }
        }
        if self.total_size >= amount && self.total_size < total {
            total = self.total_size;
        }
        total
    }
}

fn cd_command(input: &str) -> IResult<&str, Entry> {
    let (input, _) = tag("cd ")(input)?;
    let (input, path) = take_till1(|c: char| !c.is_alphanumeric() && c != '/' && c != '.')(input)?;
    Ok((
        input,
        Entry {
            name: path.to_string(),
            entry: Type::CdCommand,
            ..Entry::default()
        },
    ))
}

fn ls_command(input: &str) -> IResult<&str, Entry> {
    let (input, _) = tag("ls")(input)?;
    Ok((
        input,
        Entry {
            entry: Type::LsCommand,
            ..Entry::default()
        },
    ))
}

fn ls_entry_file(input: &str) -> IResult<&str, Entry> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, path) = take_till1(|c: char| !c.is_alphanumeric() && c != '/' && c != '.')(input)?;

    Ok((
        input,
        Entry {
            name: path.to_string(),
            entry: Type::File,
            size: size.parse().unwrap(),
            ..Entry::default()
        },
    ))
}
fn ls_entry_dir(input: &str) -> IResult<&str, Entry> {
    let (input, _) = tag("dir ")(input)?;
    let (input, path) = take_till1(|c: char| !c.is_alphanumeric() && c != '/' && c != '.')(input)?;

    Ok((
        input,
        Entry {
            name: path.to_string(),
            entry: Type::Dir,
            ..Entry::default()
        },
    ))
}

fn ls_entry(input: &str) -> IResult<&str, Entry> {
    alt((ls_entry_file, ls_entry_dir))(input)
}

fn all_command(input: &str) -> IResult<&str, Entry> {
    let (input, _) = tag("$ ")(input)?;
    alt((ls_command, cd_command))(input)
}

fn all_input(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list1(newline, alt((all_command, ls_entry)))(input)
}

fn solve(mut input: impl BufRead) -> (usize, usize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, entries) = all_input(&buffer).unwrap();

    let mut cur_dir = vec!["/".to_string()];
    let mut fs = FS {
        children: HashMap::new(),
        files: HashMap::new(),
        total_size: 0,
    };

    for e in entries {
        match e.entry {
            Type::File => {
                fs.insert_file(&cur_dir, e.name, e.size);
            }
            Type::Dir => {
                // nothing
            }
            Type::CdCommand => {
                if e.name == ".." {
                    cur_dir.pop();
                } else if e.name == "/" {
                    cur_dir = vec!["/".to_string()]
                } else {
                    cur_dir.push(e.name);
                };
            }
            Type::LsCommand => {
                // nothing
            }
        }
    }

    let part1 = fs.sum_small();
    let mut part2 = 0;

    let cur_free = 70000000 - fs.total_size;
    if cur_free <= 30000000 {
        let missing = 30000000 - cur_free;
        part2 = fs.pick_closest_ge(missing);
    }

    (part1, part2)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cd() {
        assert_eq!(
            all_command("$ cd /"),
            Ok((
                "",
                Entry {
                    entry: Type::CdCommand,
                    name: "/".to_string(),
                    ..Entry::default()
                }
            ))
        );
    }

    #[test]
    fn parse_ls() {
        assert_eq!(
            all_command("$ ls"),
            Ok((
                "",
                Entry {
                    entry: Type::LsCommand,
                    ..Entry::default()
                }
            ))
        );
    }

    #[test]
    fn parse_ls_file() {
        assert_eq!(
            ls_entry("12345 foo"),
            Ok((
                "",
                Entry {
                    name: "foo".to_string(),
                    size: 12345,
                    entry: Type::File,
                    ..Entry::default()
                }
            ))
        );
    }

    #[test]
    fn parse_ls_dir() {
        assert_eq!(
            ls_entry("dir foo"),
            Ok((
                "",
                Entry {
                    name: "foo".to_string(),
                    entry: Type::Dir,
                    ..Entry::default()
                }
            ))
        );
    }

    #[test]
    fn test_sample() {
        let sample = "$ cd /
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
        assert_eq!(solve(sample.as_bytes()), (95437, 24933642));
    }
}
