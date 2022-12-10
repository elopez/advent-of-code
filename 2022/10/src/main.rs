extern crate nom;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::*, combinator::opt,
    multi::separated_list1, IResult,
};
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Op {
    Addx(isize),
    Noop,
}

fn parse_noop(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Op::Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("addx")(input)?;
    let (input, _) = space1(input)?;
    let (input, opt_sign) = opt(alt((char('+'), char('-'))))(input)?;
    let sign = match opt_sign {
        Some('+') => 1,
        Some('-') => -1,
        _ => 1,
    };
    let (input, many) = digit1(input)?;
    Ok((input, Op::Addx(sign * many.parse::<isize>().unwrap())))
}

fn parse_cmd(input: &str) -> IResult<&str, Op> {
    alt((parse_noop, parse_addx))(input)
}

fn all_input(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(newline, parse_cmd)(input)
}

struct State {
    s: Vec<isize>,
    t: usize,
}

impl State {
    fn new() -> Self {
        Self { s: vec![1], t: 0 }
    }

    fn run(&mut self, op: Op) {
        match op {
            Op::Noop => {
                self.s.push(self.s[self.t]);
                self.t += 1;
            }
            Op::Addx(n) => {
                self.s.push(self.s[self.t]);
                self.s.push(self.s[self.t] + n);
                self.t += 2;
            }
        }
    }

    fn get(&self, t: usize) -> isize {
        self.s[t]
    }
}

fn execute_cmds(cmds: Vec<Op>) -> State {
    let mut s = State::new();

    for c in cmds {
        s.run(c);
    }

    s
}

fn solve(mut input: impl BufRead) -> isize {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, cmds) = all_input(&buffer).unwrap();

    let s = execute_cmds(cmds);

    let mut strength = 0;
    for p in vec![20, 60, 100, 140, 180, 220] {
        strength += p as isize * s.get(p - 1);
    }

    let mut drawing = String::new();
    for pixel in 0..240 {
        let sprite_center = s.get(pixel as usize);
        let c = if sprite_center >= pixel % 40 - 1 && sprite_center <= pixel % 40 + 1 {
            '#'
        } else {
            '.'
        };
        println!(
            "cycle {} sprite center {} -> {}",
            pixel + 1,
            sprite_center,
            c
        );
        drawing.push(c);
    }

    println!("{}", &drawing[0..40]);
    println!("{}", &drawing[40..80]);
    println!("{}", &drawing[80..120]);
    println!("{}", &drawing[120..160]);
    println!("{}", &drawing[160..200]);
    println!("{}", &drawing[200..240]);

    strength
}

fn main() {
    let p1 = solve(io::stdin().lock());
    println!("Total: {p1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmd() {
        assert_eq!(parse_cmd("noop"), Ok(("", Op::Noop)));
        assert_eq!(parse_cmd("addx 5"), Ok(("", Op::Addx(5))));
        assert_eq!(parse_cmd("addx -30"), Ok(("", Op::Addx(-30))));
    }

    #[test]
    fn test_sample() {
        let sample = "addx 15
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
        assert_eq!(solve(sample.as_bytes()), 13140);
    }
}
