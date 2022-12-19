extern crate nom;
use convolve2d::{convolve2d, DynamicMatrix, Matrix};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::io::{self, BufRead};
use std::str::FromStr;
use std::{cmp::max, collections::HashSet};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point(usize, usize, usize);

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, point) = separated_list1(tag(","), parse_number)(input)?;
    Ok((input, (Point(point[0], point[1], point[2]))))
}

fn all_input(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(newline, parse_point)(input)
}

fn solve(mut input: impl BufRead) -> (i32, i32) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, points) = all_input(&buffer).unwrap();

    let (max_x, max_y, max_z) = points.iter().fold((0, 0, 0), |acc, e| {
        (
            max(acc.0, e.0 + 10),
            max(acc.1, e.1 + 10),
            max(acc.2, e.2 + 10),
        )
    });

    let mut matrix = vec![vec![vec![0; max_z]; max_y]; max_x];
    for m in points.iter() {
        matrix[m.0 + 5][m.1 + 5][m.2 + 5] = 1;
    }

    let mut part1 = 0;

    let kernel_yz = DynamicMatrix::new(3, 3, vec![0, -1, 0, -1, 4, -1, 0, -1, 0]).unwrap();
    for slice in matrix.clone().into_iter() {
        let s = slice.into_iter().flatten().collect::<Vec<i32>>();
        let mat = DynamicMatrix::new(max_z, max_y, s).unwrap();
        let conv = convolve2d(&mat, &kernel_yz);
        part1 += conv
            .get_data()
            .into_iter()
            .filter_map(|c| if *c > 0 { Some(*c) } else { None })
            .sum::<i32>();
    }

    let kernel_xz = DynamicMatrix::new(3, 3, vec![0, -1, 0, 0, 2, 0, 0, -1, 0]).unwrap();
    for i_y in 0..max_y {
        let mut slice = vec![vec![0; max_z]; max_x];
        for i_x in 0..max_x {
            for i_z in 0..max_z {
                slice[i_x][i_z] = matrix[i_x][i_y][i_z];
            }
        }
        let s = slice.into_iter().flatten().collect::<Vec<i32>>();
        let mat = DynamicMatrix::new(max_z, max_x, s).unwrap();
        let conv = convolve2d(&mat, &kernel_xz);
        part1 += conv
            .get_data()
            .into_iter()
            .filter_map(|c| if *c > 0 { Some(*c) } else { None })
            .sum::<i32>();
    }

    (part1, 0)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("18,8,12"), Ok(("", Point(18, 8, 12))));
    }

    #[test]
    fn test_sample() {
        let sample = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        assert_eq!(solve(sample.as_bytes()), (64, 0))
    }
}
