use std::cmp::{max, min};
use std::io::{self, BufRead};

fn scenic_score(matrix: &Vec<Vec<i64>>, xp: usize, yp: usize) -> i64 {
    let mut c1 = 0;
    for x in (0..xp).rev() {
        c1 += 1;
        if matrix[x][yp] >= matrix[xp][yp] {
            break;
        }
    }

    let mut c2 = 0;
    for x in xp + 1..matrix.len() {
        c2 += 1;
        if matrix[x][yp] >= matrix[xp][yp] {
            break;
        }
    }

    let mut c3 = 0;
    for y in (0..yp).rev() {
        c3 += 1;
        if matrix[xp][y] >= matrix[xp][yp] {
            break;
        }
    }

    let mut c4 = 0;
    for y in yp + 1..matrix[0].len() {
        c4 += 1;
        if matrix[xp][y] >= matrix[xp][yp] {
            break;
        }
    }

    c1 * c2 * c3 * c4
}

fn visible_map(matrix: &Vec<Vec<i64>>) -> Vec<Vec<bool>> {
    let width = matrix[0].len();
    let height = matrix.len();
    let mut v: Vec<Vec<bool>> = std::iter::repeat(vec![false; width]).take(height).collect();
    let mut hh: Vec<Vec<i64>> = std::iter::repeat(vec![0; width]).take(height).collect();
    let mut hv: Vec<Vec<i64>> = std::iter::repeat(vec![0; width]).take(height).collect();

    for i in 0..width {
        v[0][i] = true;
        hh[0][i] = matrix[0][i];
        hv[0][i] = matrix[0][i];
        v[height - 1][i] = true;
        hh[height - 1][i] = matrix[height - 1][i];
        hv[height - 1][i] = matrix[height - 1][i];
    }
    for i in 0..height {
        v[i][0] = true;
        hh[i][0] = matrix[i][0];
        hv[i][0] = matrix[i][0];
        v[i][width - 1] = true;
        hh[i][width - 1] = matrix[i][width - 1];
        hv[i][width - 1] = matrix[i][width - 1];
    }

    for x in 1..height - 1 {
        for y in 1..width - 1 {
            hh[x][y] = max(matrix[x][y], hh[x][y - 1]);
            hv[x][y] = max(matrix[x][y], hv[x - 1][y]);
            let minh = min(hv[x - 1][y], hh[x][y - 1]);
            if minh >= matrix[x][y] {
                v[x][y] = false;
            } else {
                v[x][y] = true;
            }
        }
    }

    for x in (1..height - 1).rev() {
        for y in (1..width - 1).rev() {
            hh[x][y] = max(matrix[x][y], hh[x][y + 1]);
            hv[x][y] = max(matrix[x][y], hv[x + 1][y]);
            let minh = min(hv[x + 1][y], hh[x][y + 1]);
            if minh < matrix[x][y] {
                v[x][y] = true;
            }
        }
    }

    v
}

fn solve(input: impl BufRead) -> (i64, i64) {
    let lines = input.lines();
    let matrix = lines
        .map_while(|p| {
            if p.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(p.unwrap())
            }
        })
        .map(|v| {
            v.split("")
                .collect::<Vec<&str>>()
                .split_first()
                .unwrap()
                .1
                .split_last()
                .unwrap()
                .1
                .iter()
                .map(|p| p.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    let v = visible_map(&matrix);

    let part1 = v
        .iter()
        .map(|r| r.iter().map(|v| if *v { 1 } else { 0 }).sum::<i64>())
        .sum();

    let width = matrix[0].len();
    let height = matrix.len();
    let mut best_scenic = 0;
    for x in 1..height {
        for y in 1..width {
            let score = scenic_score(&matrix, x, y);
            best_scenic = max(best_scenic, score);
        }
    }

    let part2 = best_scenic;

    (part1, part2)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = "30373
25512
65332
33549
35390
";
        let result = super::solve(case.as_bytes());
        assert_eq!(result, (21, 8));
    }
}
