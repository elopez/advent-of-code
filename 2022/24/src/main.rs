use pathfinding::prelude::dijkstra;
use std::io::{self, BufRead};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, usize);

impl Pos {
    fn move_to_person(
        &self,
        d_x: isize,
        d_y: isize,
        height: usize,
        width: usize,
        m: &[Vec<u8>],
    ) -> Option<Self> {
        let x: isize = self.0 as isize + d_x;
        let y: isize = self.1 as isize + d_y;
        if x < 0 || y < 0 || x > height as isize - 1 || y > width as isize - 1 {
            None
        } else {
            let target = Pos(x as usize % height, y as usize % width, self.2 + 1);
            match m[target.0][target.1] {
                b'#' => None,
                _ => Some(target),
            }
        }
    }

    fn move_to_wind(&self, d_x: isize, d_y: isize, height: usize, width: usize) -> Self {
        let mut x: isize = (self.0 as isize + d_x - 1) % (height as isize);
        let mut y: isize = (self.1 as isize + d_y - 1) % (width as isize);
        x += height as isize;
        y += width as isize;
        Pos(x as usize % height + 1, y as usize % width + 1, self.2)
    }

    fn successors(&self, m: &Vec<Vec<u8>>) -> Vec<(Pos, usize)> {
        let height = m.len();
        let width = m[0].len();
        let states = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];
        let moves = vec![(0, 1, b'<'), (0, -1, b'>'), (1, 0, b'^'), (-1, 0, b'v')];

        let mut pos = vec![];
        'moves: for (p_x, p_y) in states {
            let target: Pos;
            if let Some(x) = self.move_to_person(p_x, p_y, height, width, m) {
                target = x;
            } else {
                continue;
            };

            // Walking to goal or start is always ok
            if target.0 == height - 1 || target.0 == 0 {
                pos.push(target);
                continue;
            }

            for (d_x, d_y, wind) in moves.iter() {
                let past = target.move_to_wind(
                    d_x * (target.2 as isize),
                    d_y * (target.2 as isize),
                    height - 2,
                    width - 2,
                );
                if m[past.0][past.1] == *wind {
                    continue 'moves;
                }
            }

            pos.push(target);
        }

        pos.into_iter().map(|p| (p, 1)).collect()
    }
}

fn solve(input: impl BufRead) -> (usize, usize) {
    let lines = input.lines();
    let mapvec = lines
        .map_while(|p| {
            if p.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(p.unwrap())
            }
        })
        .map(|s| s.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let start_y = mapvec[0]
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == b'.' { Some(i) } else { None })
        .sum();
    let goal_y = mapvec[mapvec.len() - 1]
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == b'.' { Some(i) } else { None })
        .sum();
    let start = Pos(0, start_y, 0);
    let goal = Pos(mapvec.len() - 1, goal_y, 0);

    // part 1
    let p1 = dijkstra(
        &start,
        |p| p.successors(&mapvec),
        |p| p.0 == goal.0 && p.1 == goal.1,
    );
    let (path_p1, score_p1) = p1.unwrap();

    // part 2
    let p2_return = dijkstra(
        &path_p1[path_p1.len() - 1],
        |p| p.successors(&mapvec),
        |p| p.0 == start.0 && p.1 == start.1,
    );
    let (path_return, score_return) = p2_return.unwrap();

    let p2 = dijkstra(
        &path_return[path_return.len() - 1],
        |p| p.successors(&mapvec),
        |p| p.0 == goal.0 && p.1 == goal.1,
    );
    let (_, score_p2) = p2.unwrap();

    (score_p1, score_p1 + score_return + score_p2)
}

fn main() {
    let (score_p1, score_p2) = solve(io::stdin().lock());
    println!("Total score: {score_p1} / {score_p2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let result = super::solve(case.as_bytes());
        assert_eq!(result, (18, 54));
    }
}
