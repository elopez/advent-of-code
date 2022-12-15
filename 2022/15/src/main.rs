extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point(isize, isize);

impl Point {
    fn taxicab(&self, other: &Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    let (input, opt_sign) = opt(alt((char('+'), char('-'))))(input)?;
    let sign = match opt_sign {
        Some('+') => 1,
        Some('-') => -1,
        _ => 1,
    };
    let (input, nr) = map_res(digit1, isize::from_str)(input)?;
    Ok((input, nr * sign))
}

fn parse_pair(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, (_, sx, _, sy, _, bx, _, by)) = tuple((
        tag("Sensor at x="),
        parse_number,
        tag(", y="),
        parse_number,
        tag(": closest beacon is at x="),
        parse_number,
        tag(", y="),
        parse_number,
    ))(input)?;
    Ok((input, (Point(sx, sy), Point(bx, by))))
}

fn all_input(input: &str) -> IResult<&str, Vec<(Point, Point)>> {
    separated_list1(newline, parse_pair)(input)
}

fn solve(mut input: impl BufRead, y_check: isize, max_range: usize) -> (usize, usize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, pairs) = all_input(&buffer).unwrap();

    let min_x = pairs.iter().map(|x| x.0 .0).min().unwrap();
    let max_x = pairs.iter().map(|x| x.0 .0).max().unwrap();
    let max_d = pairs
        .iter()
        .map(|x| x.0.taxicab(&x.1) - (x.0 .1 - y_check).abs())
        .max()
        .unwrap();

    let mut impossible: HashSet<Point> = HashSet::new();
    for x in min_x - max_d - 5..max_x + max_d + 5 {
        let test = Point(x, y_check);
        let blocked: HashSet<Point> = HashSet::from_iter(pairs.iter().filter_map(|p| {
            if test.taxicab(&p.0) <= p.1.taxicab(&p.0) && p.1 != test {
                Some(test.clone())
            } else {
                None
            }
        }));
        for b in blocked {
            impossible.insert(b);
        }
    }

    let (mut found_x, mut found_y) = (0, 0);
    'outer: for x in 0..=max_range {
        let mut y: usize = 0;
        'inner: loop {
            if y > max_range {
                break;
            }

            let test = Point(x.try_into().unwrap(), y.try_into().unwrap());
            for p in pairs.iter() {
                let total = p.1.taxicab(&p.0);
                let test_dist = test.taxicab(&p.0);
                if test_dist < total {
                    let extra: usize = (total - test_dist).try_into().unwrap();
                    y += extra;
                    continue 'inner;
                } else if test_dist == total || test == p.1 {
                    y += 1;
                    continue 'inner;
                }
            }

            (found_x, found_y) = (x, y);
            break 'outer;
        }
    }

    (impossible.len(), found_x * 4000000 + found_y)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock(), 2000000, 4000000);
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(
            parse_pair("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Ok(("", (Point(2, 18), Point(-2, 15))))
        );
    }

    #[test]
    fn test_sample() {
        let sample = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(solve(sample.as_bytes(), 10, 20), (26, 56000011))
    }
}
