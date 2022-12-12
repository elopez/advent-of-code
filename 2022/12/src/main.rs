use pathfinding::prelude::dijkstra;
use std::io::{self, BufRead};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos<'a>(usize, usize, &'a Vec<Vec<u8>>);

impl<'a> Pos<'a> {
    fn successors(&self) -> Vec<(Pos<'a>, usize)> {
        let &Pos(x, y, m) = self;
        let mut pos = vec![];
        if x > 0 && m[x][y] <= m[x - 1][y] + 1 {
            pos.push(Pos(x - 1, y, m));
        }
        if y > 0 && m[x][y] <= m[x][y - 1] + 1 {
            pos.push(Pos(x, y - 1, m));
        }
        if x + 1 < m.len().try_into().unwrap() && m[x][y] <= m[x + 1][y] + 1 {
            pos.push(Pos(x + 1, y, m));
        }
        if y + 1 < m[0].len().try_into().unwrap() && m[x][y] <= m[x][y + 1] + 1 {
            pos.push(Pos(x, y + 1, m));
        }
        pos.into_iter().map(|p| (p, 1)).collect()
    }
}

fn solve(input: impl BufRead) -> (usize, usize) {
    let lines = input.lines();
    let mut mapvec = lines
        .map_while(|p| {
            if p.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(p.unwrap())
            }
        })
        .map(|s| s.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let tmp = vec![];
    let mut start = Pos(0, 0, &tmp);
    let mut goal = Pos(0, 0, &tmp);

    for x in 0..mapvec.len() {
        for y in 0..mapvec[x].len() {
            match mapvec[x][y] as char {
                'E' => {
                    mapvec[x][y] = 'z' as u8;
                    goal.0 = x;
                    goal.1 = y;
                }
                'S' => {
                    mapvec[x][y] = 'a' as u8;
                    start.0 = x;
                    start.1 = y;
                }
                _ => (),
            }
        }
    }

    start.2 = &mapvec;
    goal.2 = &mapvec;

    let p1 = dijkstra(&goal, |p| p.successors(), |p| *p == start);
    let p2 = dijkstra(&goal, |p| p.successors(), |p| p.2[p.0][p.1] == 'a' as u8);

    (p1.unwrap().1, p2.unwrap().1)
}

fn main() {
    let (score_p1, score_p2) = solve(io::stdin().lock());
    println!("Total score: {score_p1} / {score_p2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let result = super::solve(case.as_bytes());
        assert_eq!(result, (31, 29));
    }
}
