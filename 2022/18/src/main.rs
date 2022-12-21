extern crate nom;
use convolve2d::{convolve2d, DynamicMatrix, Matrix};
use nom::{
    bytes::complete::tag, character::complete::*, combinator::map_res, multi::separated_list1,
    IResult,
};
use std::cmp::max;
use std::str::FromStr;
use std::{
    collections::VecDeque,
    hash::Hash,
    io::{self, BufRead},
};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point(usize, usize, usize);

impl Point {
    fn neighbors(&self, max_x: usize, max_y: usize, max_z: usize) -> Vec<Self> {
        let mut set = vec![];
        for d_x in [-1, 1] {
            let c_x = self.0 as isize + d_x;
            if c_x < 0 || c_x as usize >= max_x {
                continue;
            }
            set.push(Point(c_x as usize, self.1, self.2))
        }

        for d_y in [-1, 1] {
            let c_y = self.1 as isize + d_y;
            if c_y < 0 || c_y as usize >= max_y {
                continue;
            }
            set.push(Point(self.0, c_y as usize, self.2))
        }

        for d_z in [-1, 1] {
            let c_z = self.2 as isize + d_z;
            if c_z < 0 || c_z as usize >= max_z {
                continue;
            }
            set.push(Point(self.0, self.1, c_z as usize))
        }

        set
    }
}

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

    let mut external_matrix = vec![vec![vec![0; max_z]; max_y]; max_x];
    let mut external_seen = vec![vec![vec![false; max_z]; max_y]; max_x];
    let mut pending: VecDeque<Point> = VecDeque::from_iter([Point(0, 0, 0)]);
    while !pending.is_empty() {
        let p = pending.pop_front().unwrap();
        if external_seen[p.0][p.1][p.2] {
            continue;
        }
        external_seen[p.0][p.1][p.2] = true;

        if matrix[p.0][p.1][p.2] == 1 {
            continue;
        } else {
            external_matrix[p.0][p.1][p.2] = 1
        }

        for n in p.neighbors(max_x, max_y, max_z) {
            if external_seen[n.0][n.1][n.2] {
                continue;
            }

            if matrix[n.0][n.1][n.2] == 0 {
                pending.push_back(n);
            }
        }
    }

    let part1 = count_exposed_faces(&matrix, max_x, max_y, max_z);

    // TODO: find out why this is not correct
    // let external = max_x * max_y * 2 + max_x * max_z * 2 + max_y * max_z * 2;
    let external_full = vec![vec![vec![1; max_z]; max_y]; max_x];
    let external = count_exposed_faces(&external_full, max_x, max_y, max_z);
    let part2 = count_exposed_faces(&external_matrix, max_x, max_y, max_z) - external;

    (part1, part2)
}

fn count_exposed_faces(matrix: &[Vec<Vec<i32>>], max_x: usize, max_y: usize, max_z: usize) -> i32 {
    let mut total = 0;

    let kernel_yz = DynamicMatrix::new(3, 3, vec![0, -1, 0, -1, 4, -1, 0, -1, 0]).unwrap();
    for slice in matrix.iter().cloned() {
        let s = slice.into_iter().flatten().collect::<Vec<i32>>();
        let mat = DynamicMatrix::new(max_z, max_y, s).unwrap();
        let conv = convolve2d(&mat, &kernel_yz);
        total += conv
            .get_data()
            .iter()
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
        total += conv
            .get_data()
            .iter()
            .filter_map(|c| if *c > 0 { Some(*c) } else { None })
            .sum::<i32>();
    }

    total
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

        assert_eq!(solve(sample.as_bytes()), (64, 58))
    }
}
