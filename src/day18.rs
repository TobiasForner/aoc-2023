use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

use std::{
    collections::{HashSet, VecDeque},
    ops::Index,
    str::FromStr,
};

use crate::util::{self, Direction};

#[derive(Clone, Debug)]
struct Trench {
    direction: Direction,
    length: i64,
    hex_length: i64,
    hex_direction: Direction,
}

impl FromStr for Trench {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use Direction::*;
        let parts: Vec<&str> = s.split(' ').collect();
        let direction = match parts[0] {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => bail!(""),
        };
        let hex_part = parts[2][2..parts[2].len() - 1].to_string();
        let hex_digits = hex_part
            .chars()
            .map(|c| c.to_digit(16).unwrap() as usize)
            .collect_vec();

        let base: usize = 16;
        let hex_length: i64 = (1..=5)
            .map(|p| {
                let pos = hex_digits.len() - p - 1;
                let res: i64 = (base.pow((p - 1) as u32) * hex_digits[pos])
                    .try_into()
                    .unwrap();
                res
            })
            .sum();
        let hex_direction = match hex_digits[hex_digits.len() - 1] {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => bail!(""),
        };
        let length: i64 = parts[1].parse()?;
        Ok(Self {
            direction,
            length,
            hex_length,
            hex_direction,
        })
    }
}

impl Trench {
    fn dir(&self, hex: bool) -> Direction {
        if hex {
            self.hex_direction.clone()
        } else {
            self.direction.clone()
        }
    }

    fn len(&self, hex: bool) -> i64 {
        if hex {
            self.hex_length
        } else {
            self.length
        }
    }
}

fn move_n_in_dir(x: i64, y: i64, dir: &Direction, n: i64) -> (i64, i64) {
    use Direction::*;
    match dir {
        Left => (x - n, y),
        Right => (x + n, y),
        Up => (x, y - n),
        Down => (x, y + n),
    }
}

fn parse_trenches(text: &str) -> Result<Vec<Trench>> {
    text.lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Trench>>>()
}

fn compute_positions(trenches: &Vec<Trench>, hex: bool) -> Vec<(i64, i64)> {
    let mut res = vec![];
    let mut x = 0;
    let mut y = 0;
    res.push((x, y));
    trenches.iter().for_each(|t| {
        let tmp = move_n_in_dir(x, y, &t.dir(hex), t.len(hex));
        x = tmp.0;
        y = tmp.1;
        res.push((x, y));
    });
    res
}

fn compute_trench_positions(trenches: &Vec<Trench>, hex: bool) -> Vec<(Trench, i64, i64)> {
    let mut res = vec![];
    let mut x = 0;
    let mut y = 0;
    trenches.iter().for_each(|t| {
        let tmp = move_n_in_dir(x, y, &t.dir(hex), t.len(hex));

        res.push((t.clone(), x, y));
        x = tmp.0;
        y = tmp.1;
    });
    res
}

fn trench_contains_rect(
    start_x: i64,
    start_y: i64,
    trench: &Trench,
    rect: &Rect,
    hex: bool,
) -> bool {
    use Direction::*;
    let after = move_n_in_dir(start_x, start_y, &trench.dir(hex), trench.len(hex));
    let res = match trench.dir(hex) {
        Up => {
            rect.is_ver_line()
                && start_y + 1 >= rect.bottom
                && after.1 <= rect.top
                && start_x == rect.left
        }
        Down => {
            rect.is_ver_line()
                && start_y <= rect.top
                && after.1 + 1 >= rect.bottom
                && start_x == rect.left
        }
        Left => {
            rect.is_hor_line()
                && start_x + 1 >= rect.right
                && after.0 <= rect.left
                && start_y == rect.top
        }
        Right => {
            rect.is_hor_line()
                && start_x <= rect.left
                && after.0 + 1 >= rect.right
                && start_y == rect.top
        }
    };
    res
}

fn is_wall(rect: &Rect, tpositions: &Vec<(Trench, i64, i64)>, hex: bool) -> bool {
    /*if rect.left == 2 && rect.top == 5 {
        println!("here");
    }*/
    let res = tpositions
        .iter()
        .any(|(t, x, y)| trench_contains_rect(*x, *y, t, rect, hex));
    //println!("{rect:?} is wall: {res}");
    res
}

fn parse_input(text: &str) -> Result<Vec<Vec<char>>> {
    let trenches = parse_trenches(text)?;
    let positions = compute_positions(&trenches, false);

    let min_x = positions.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = positions.iter().map(|(_, y)| *y).min().unwrap();
    let res = positions
        .into_iter()
        .map(|(x, y)| ((x - min_x) as usize, (y - min_y) as usize))
        .collect_vec();

    let width = res.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let height = res.iter().map(|(_, y)| *y).max().unwrap() + 1;

    let mut grid = (0..height)
        .map(|_| (0..width).map(|_| '.').collect_vec())
        .collect_vec();

    let mut x = res[0].0;
    let mut y = res[0].1;
    grid[y][x] = '#';
    trenches.iter().for_each(|t| {
        (1..=t.length).for_each(|_| {
            let tmp = move_n_in_dir(x as i64, y as i64, &t.direction, 1);
            x = tmp.0 as usize;
            y = tmp.1 as usize;
            grid[y][x] = '#';
        })
    });
    Ok(grid)
}

fn part1_old(text: &str) -> Result<()> {
    let mut grid = parse_input(text)?;

    mark_inner(&mut grid);

    let res = grid
        .iter()
        .map(|l| l.iter().filter(|c| **c == '#'))
        .flatten()
        .count();
    println!("part 1: {res}");
    Ok(())
}

fn reached_by(pos: (usize, usize), map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let x = pos.0;
    let y = pos.1;
    let mut res = HashSet::new();
    let nx = x.saturating_sub(1);

    if map[y][nx] == '.' {
        res.insert((nx, y));
    }
    let nx = x.saturating_add(1).min(map[0].len() - 1);
    if map[y][nx] == '.' {
        res.insert((nx, y));
    }
    let ny = y.saturating_sub(1);
    if map[ny][x] == '.' {
        res.insert((x, ny));
    }
    let ny = y.saturating_add(1).min(map.len() - 1);
    if map[ny][x] == '.' {
        res.insert((x, ny));
    }
    res
}

#[derive(Debug)]
struct Rect {
    left: i64,   //incl
    right: i64,  //excl
    top: i64,    //incl
    bottom: i64, //exclusive
}

impl Rect {
    fn area(&self) -> i128 {
        let width = (self.right - self.left) as i128;
        let height = (self.bottom - self.top) as i128;
        width * height
    }

    fn is_hor_line(&self) -> bool {
        self.bottom - self.top == 1
    }

    fn is_ver_line(&self) -> bool {
        self.right - self.left == 1
    }
}

fn mark_inner(grid: &mut Vec<Vec<char>>) {
    let mut todo: VecDeque<(usize, usize)> = grid
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == '.')
                .map(move |(x, _)| (x, y))
        })
        .flatten()
        .collect();

    let mut visited = HashSet::new();
    while let Some(pos) = todo.pop_front() {
        let mut to_ex = VecDeque::new();
        to_ex.push_back(pos);
        let mut new_visited = HashSet::new();
        while let Some(pos) = to_ex.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            let next = reached_by(pos, &grid);
            let new = next.difference(&visited);
            new.for_each(|e| to_ex.push_back(*e));
            visited.insert(pos);
            new_visited.insert(pos);
        }
        if !new_visited
            .iter()
            .any(|(x, y)| *x == 0 || *x == grid[0].len() - 1 || *y == 0 || *y == grid.len() - 1)
        {
            //println!("{new_visited:?}");
            new_visited.iter().for_each(|(x, y)| grid[*y][*x] = '#');
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    let v: Vec<String> = map.iter().map(|l| l.iter().collect::<String>()).collect();
    let text = v.join("\n");
    println!("{text}");
}

fn print_rects(rects: &Vec<Rect>) {
    let text = rects.iter().map(|r| format!("{r:?}")).join("\n");
    println!("{text}");
}

fn solve(text: &str, hex: bool) -> Result<i128> {
    let trenches = parse_trenches(text)?;
    //println!("{trenches:?}");
    let positions = compute_positions(&trenches, hex);
    //println!("{positions:?}");

    let xs = positions
        .iter()
        .map(|(x, _)| [*x, *x + 1])
        .flatten()
        .sorted()
        .unique()
        .collect_vec();
    //let xl = xs.len();
    //xs[xl - 1] += 1;
    let ys = positions
        .iter()
        .map(|(_, y)| [*y, *y + 1])
        .flatten()
        .sorted()
        .unique()
        .collect_vec();
    //let yl = ys.len();
    //ys[yl - 1] += 1;
    let mut rects = xs
        .windows(2)
        .map(|xw| {
            ys.windows(2).map(|yw| Rect {
                left: xw[0],
                right: xw[1],
                top: yw[0],
                bottom: yw[1],
            })
        })
        .flatten()
        .collect_vec();

    //sort rects appropriatly
    rects.sort_by(|r1, r2| (r1.top, r1.left).cmp(&(r2.top, r2.left)));
    let rects = rects;

    //print_rects(&rects);

    let tpositions = compute_trench_positions(&trenches, hex);
    let rect_is_wall: Vec<bool> = rects
        .iter()
        .map(|r| is_wall(r, &tpositions, hex))
        .collect_vec();

    //project each rect onto a single point

    let width = rects.iter().map(|r| r.left).unique().count();
    let height = rects.len() / width;
    let mut grid = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let pos = y * width + x;
                    if rect_is_wall[pos] {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    // classify points
    //print_map(&grid);
    //println!("------------------");
    mark_inner(&mut grid);

    //print_map(&grid);
    // use point classification for rects

    let res: i128 = grid
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| (x, y))
        })
        .flatten()
        .map(|(x, y)| rects.index(y * width + x).area())
        .sum();

    Ok(res)
}

fn part1(text: &str) -> Result<()> {
    let res = solve(text, false)?;
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let res = solve(text, true)?;
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day18.txt").expect("expected readable file");
    let text = util::read_input_file(18).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
