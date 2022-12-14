extern crate nom;
use nom::{
    bytes::complete::tag, character::complete::*, combinator::map_res, multi::separated_list1,
    sequence::tuple, IResult,
};
use std::iter::zip;
use std::str::FromStr;
use std::{
    cmp::{max, min},
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Clone)]
struct Point(usize, usize);

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(input)
}

fn parse_pair(input: &str) -> IResult<&str, Point> {
    let (input, (a, _, b)) = tuple((parse_number, tag(","), parse_number))(input)?;
    Ok((input, Point(b, a)))
}

fn parse_list(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_pair)(input)
}

fn all_input(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(newline, parse_list)(input)
}

fn solve(mut input: impl BufRead) -> (usize, usize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, rocks) = all_input(&buffer).unwrap();

    let max_x = rocks
        .iter()
        .map(|v| v.iter().map(|p| p.0).max().unwrap_or(0))
        .max()
        .unwrap();
    let max_y = rocks
        .iter()
        .map(|v| v.iter().map(|p| p.1).max().unwrap_or(0))
        .max()
        .unwrap();

    let mut map = generate_map(max_x + 3, max_y + max_x + 3, &rocks);
    let (count_p1, map_p1) = run_simulation(map);
    // _print_map(&map_p1);

    map = patch_map(max_x + 2, map_p1);
    let (count_p2, _map_p2) = run_simulation(map);
    //_print_map(&_map_p2);

    (count_p1, count_p1 + count_p2 + 1)
}

fn run_simulation(mut map: Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let origin = Point(0, 500);
    let mut count = 0;
    loop {
        match simulate_sand_drop(&origin, map) {
            (m, Some(target)) => {
                map = m;
                if target == origin {
                    break;
                }
            }
            (m, None) => {
                map = m;
                break;
            }
        }
        count += 1;
    }
    (count, map)
}

fn generate_map(max_x: usize, max_y: usize, rocks: &[Vec<Point>]) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; max_y]; max_x];
    for rock in rocks.iter() {
        for (src, dst) in zip(rock.iter(), rock.iter().skip(1)) {
            let max_x = max(src.0, dst.0);
            let min_x = min(src.0, dst.0);
            let max_y = max(src.1, dst.1);
            let min_y = min(src.1, dst.1);
            for r in map.iter_mut().take(max_x + 1).skip(min_x) {
                for e in r.iter_mut().take(max_y + 1).skip(min_y) {
                    *e = '#';
                }
            }
        }
    }

    map
}

fn patch_map(line: usize, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for c in map[line].iter_mut() {
        *c = '#';
    }
    map
}

fn simulate_sand_drop(pos: &Point, mut map: Vec<Vec<char>>) -> (Vec<Vec<char>>, Option<Point>) {
    if pos.0 + 1 >= map.len() || pos.1 + 1 >= map[0].len() {
        println!("{:?}", pos);
        return (map, None);
    }

    if map[pos.0 + 1][pos.1] == '.' {
        simulate_sand_drop(&Point(pos.0 + 1, pos.1), map)
    } else if map[pos.0 + 1][pos.1 - 1] == '.' {
        simulate_sand_drop(&Point(pos.0 + 1, pos.1 - 1), map)
    } else if map[pos.0 + 1][pos.1 + 1] == '.' {
        simulate_sand_drop(&Point(pos.0 + 1, pos.1 + 1), map)
    } else {
        map[pos.0][pos.1] = 'o';
        (map, Some(pos.clone()))
    }
}

fn _print_map(map: &[Vec<char>]) {
    let mut map = map.to_owned();
    map[0][500] = 'S';

    for m in map.iter() {
        println!("{}", m.iter().collect::<String>());
    }
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list() {
        assert_eq!(
            parse_list("498,4 -> 498,6 -> 496,6"),
            Ok(("", vec![Point(4, 498), Point(6, 498), Point(6, 496)]))
        );
    }

    #[test]
    fn test_sample() {
        let sample = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(solve(sample.as_bytes()), (24, 93))
    }
}
