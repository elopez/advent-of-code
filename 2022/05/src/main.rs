extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::*,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
pub struct Move {
    pub many: usize,
    pub from: usize,
    pub to: usize,
}

fn crate_box(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("[")(input)?;
    let (input, c) = alpha1(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, c))
}

fn empty_box(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, ""))
}

fn crate_or_empty(input: &str) -> IResult<&str, &str> {
    alt((crate_box, empty_box))(input)
}

fn move_str(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, many) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = digit1(input)?;
    Ok((
        input,
        Move {
            many: many.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        },
    ))
}

fn all_input(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates) = many1(terminated(
        separated_list1(tag(" "), crate_or_empty),
        newline,
    ))(input)?;
    let (input, _) = terminated(take_until("\n"), newline)(input)?;
    let (input, _) = terminated(take_until("\n"), newline)(input)?;
    let (input, moves) = many1(terminated(move_str, newline))(input)?;
    Ok((input, (crates, moves)))
}

fn solve(mut input: impl BufRead) -> (String, String) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, (tower, moves)) = all_input(&buffer).unwrap();

    let mut ordered_tower = vec![vec![]; tower[0].len()];
    for row in tower.iter().rev() {
        for (j, l) in row.iter().enumerate() {
            if *l != "" {
                ordered_tower[j].push(*l);
            }
        }
    }
    let mut ordered_tower_p2 = ordered_tower.clone();

    for m in moves.iter() {
        for _ in 0..m.many {
            let val = ordered_tower[m.from - 1].pop().unwrap();
            ordered_tower[m.to - 1].push(val);
        }
    }

    for m in moves.iter() {
        let mut tmp = vec![];
        for _ in 0..m.many {
            let val = ordered_tower_p2[m.from - 1].pop().unwrap();
            tmp.push(val);
        }

        for _ in 0..m.many {
            let val = tmp.pop().unwrap();
            ordered_tower_p2[m.to - 1].push(val);
        }
    }

    let part1 = ordered_tower
        .iter()
        .filter_map(|t| {
            if t.len() > 0 {
                Some(t[t.len() - 1])
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join("");

    let part2 = ordered_tower_p2
        .iter()
        .filter_map(|t| {
            if t.len() > 0 {
                Some(t[t.len() - 1])
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join("");

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
    fn parse_crate_or_empty() {
        assert_eq!(crate_or_empty("[X] "), Ok((" ", "X")));
        assert_eq!(crate_or_empty("    "), Ok((" ", "")));
    }

    #[test]
    fn parse_move_str() {
        assert_eq!(
            move_str("move 50 from 40 to 30"),
            Ok((
                "",
                Move {
                    many: 50,
                    from: 40,
                    to: 30
                }
            ))
        );
    }

    #[test]
    fn parse_sample() {
        let sample = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let expected_tower = vec![vec!["", "D", ""], vec!["N", "C", ""], vec!["Z", "M", "P"]];
        let expected_moves = vec![
            Move {
                many: 1,
                from: 2,
                to: 1,
            },
            Move {
                many: 3,
                from: 1,
                to: 3,
            },
            Move {
                many: 2,
                from: 2,
                to: 1,
            },
            Move {
                many: 1,
                from: 1,
                to: 2,
            },
        ];
        assert_eq!(
            all_input(sample),
            Ok(("", (expected_tower, expected_moves)))
        );

        let result = super::solve(sample.as_bytes());
        assert_eq!(result, ("CMZ".to_string(), "MCD".to_string()));
    }
}
