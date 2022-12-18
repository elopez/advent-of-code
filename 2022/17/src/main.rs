use std::cmp::max;
use std::io::{self, BufRead};
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(isize, isize);

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Debug)]
struct Rock {
    bits: Vec<Pos>,
}

impl Rock {
    fn move_rock(&self, movement: &Pos, map: &[[char; 7]]) -> Option<Self> {
        let new_rock = Rock {
            bits: self.bits.iter().map(|x| *x + *movement).collect(),
        };

        let bad: isize = new_rock
            .bits
            .iter()
            .map(|x| if x.1 < 0 || x.1 > 6 || x.0 < 0 { 1 } else { 0 })
            .sum();

        let hit: isize = new_rock
            .bits
            .iter()
            .map(|x| {
                if bad == 0 && map[x.0 as usize][x.1 as usize] != '.' {
                    1
                } else {
                    0
                }
            })
            .sum();

        if bad > 0 || hit > 0 {
            None
        } else {
            Some(new_rock)
        }
    }
}

fn _show_map(map: &[[char; 7]]) {
    for l in map.iter().rev() {
        println!("{}", l.iter().collect::<String>());
    }
    println!();
}

fn solve(input: String, take_len: usize) -> isize {
    let rock_set = vec![
        Rock {
            bits: vec![Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(0, 3)],
        },
        Rock {
            bits: vec![Pos(0, 1), Pos(1, 0), Pos(1, 1), Pos(1, 2), Pos(2, 1)],
        },
        Rock {
            bits: vec![Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2), Pos(2, 2)],
        },
        Rock {
            bits: vec![Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)],
        },
        Rock {
            bits: vec![Pos(0, 0), Pos(0, 1), Pos(1, 0), Pos(1, 1)],
        },
    ];
    let rocks = rock_set.iter().cycle();

    let movement_list = input
        .chars()
        .map(|c| match c {
            '>' => Pos(0, 1),
            '<' => Pos(0, -1),
            _ => panic!("unknown"),
        })
        .collect::<Vec<_>>();
    let mut movements = movement_list.iter().cycle();
    let move_down = Pos(-1, 0);

    let mut map = vec![['.'; 7]; take_len * 5];
    let mut spawn_point = Pos(3, 2);

    'outer: for r in rocks.take(take_len) {
        let mut rock = r.clone().move_rock(&spawn_point, &map).unwrap();
        //println!("Rock drop spawn @ {:?}: {:?}", spawn_point, rock);

        loop {
            let m = movements.next().unwrap();
            if let Some(nr) = rock.move_rock(m, &map) {
                rock = nr
            }
            //println!("Rock moved {:?}: {:?}", m, rock);
            match rock.move_rock(&move_down, &map) {
                Some(nr) => rock = nr,
                None => {
                    for Pos(x, y) in rock.bits {
                        map[x as usize][y as usize] = '#';
                        spawn_point.0 = max(spawn_point.0, x + 4);
                    }
                    //_show_map(&map);
                    continue 'outer;
                }
            }
        }
    }

    spawn_point.0 - 3
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .expect("no input?");

    let score_p1 = solve(input.clone(), 2022);
    println!("Total score P1: {score_p1}");

    //let score_p2 = solve(input, 1000000000000);
    //println!("Total score P2: {score_p2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let result = super::solve(case.to_string(), 2022);
        assert_eq!(result, 3068);
    }
}
