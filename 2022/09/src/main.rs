extern crate nom;
use nom::{character::complete::*, multi::separated_list1, IResult};
use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Side {
    Right,
    Left,
    Up,
    Down,
}

impl Side {
    fn apply(&self, t: (isize, isize)) -> (isize, isize) {
        let (x, y) = t;
        match self {
            Side::Right => (x, y + 1),
            Side::Left => (x, y - 1),
            Side::Up => (x + 1, y),
            Side::Down => (x - 1, y),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    side: Side,
    many: usize,
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, m) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, many) = digit1(input)?;
    Ok((
        input,
        Move {
            side: match m {
                "R" => Side::Right,
                "L" => Side::Left,
                "U" => Side::Up,
                "D" => Side::Down,
                _ => panic!("unknown"),
            },
            many: many.parse().unwrap(),
        },
    ))
}

fn all_input(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, parse_move)(input)
}

fn adjust_tail(h: (isize, isize), t: (isize, isize)) -> (isize, isize) {
    let (hx, hy) = h;
    let (tx, ty) = t;

    if (hx - tx).abs() < 2 && (hy - ty).abs() < 2 {
        t
    } else if (hx - tx).abs() == 2 && (hy - ty).abs() == 1 {
        ((hx + tx) / 2, hy)
    } else if (hx - tx).abs() == 1 && (hy - ty).abs() == 2 {
        (hx, (hy + ty) / 2)
    } else {
        ((hx + tx) / 2, (hy + ty) / 2)
    }
}

fn _debug_display_map(pos: &Vec<(isize, isize)>) {
    let char_map = "987654321H";
    let mut d = vec!["......".to_string(); 6];
    for (p, (a, b)) in pos.iter().rev().enumerate() {
        let au = *a as usize;
        let bu = *b as usize;
        let pu: usize = p as usize;
        d[au].replace_range(bu..bu + 1, &char_map[pu..pu + 1]);
    }
    for l in d.iter().rev() {
        println!("{}", l);
    }
    println!("");
}

fn solve(mut input: impl BufRead) -> (usize, usize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, moves) = all_input(&buffer).unwrap();

    let mut tail_1_pos = HashSet::<(isize, isize)>::new();
    let mut tail_10_pos = HashSet::<(isize, isize)>::new();

    let knots = 10;
    let mut pos = vec![(0, 0); knots];
    for m in moves {
        for _ in 0..m.many {
            pos[0] = m.side.apply(pos[0]);
            for i in 1..knots {
                pos[i] = adjust_tail(pos[i - 1], pos[i]);
            }

            //_debug_display_map(&pos);

            tail_1_pos.insert(pos[1]);
            tail_10_pos.insert(pos[knots - 1]);
        }
    }

    (tail_1_pos.len(), tail_10_pos.len())
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move() {
        assert_eq!(
            parse_move("R 4"),
            Ok((
                "",
                Move {
                    side: Side::Right,
                    many: 4
                }
            ))
        );
    }

    #[test]
    fn test_sample_1() {
        let sample = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(solve(sample.as_bytes()), (13, 1));
    }

    #[test]
    fn test_sample_2() {
        let sample = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(solve(sample.as_bytes()).1, 36);
    }
}
