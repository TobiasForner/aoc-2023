use anyhow::{Context, Result, bail};

use std::collections::{HashSet, VecDeque};

use crate::util;

fn parse_input(text: &str) -> Result<Vec<Vec<GridPosition>>> {
    text.lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    Ok(GridPosition {
                        element: Element::from_char(c)?,
                        energized: false,
                    })
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone)]
enum Element {
    Space,
    HorizontalSplitter,
    VerticalSplitter,
    SlashMirror,
    BSlashMirror,
}

#[derive(Debug, Clone)]
struct GridPosition {
    element: Element,
    energized: bool,
}

impl Element {
    fn from_char(c: char) -> Result<Self> {
        use Element::*;
        match c {
            '.' => Ok(Space),
            '/' => Ok(SlashMirror),
            '\\' => Ok(BSlashMirror),
            '|' => Ok(VerticalSplitter),
            '-' => Ok(HorizontalSplitter),
            _ => bail!("No element for {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Position {
    fn next(&self, element: &Element) -> Vec<Self> {
        use Direction::*;
        use Element::*;
        let mut res = vec![];
        match (element, self.direction) {
            (Space, d) => res.push(self.move_in_dir(d)),
            (SlashMirror, Left) => res.push(self.move_in_dir(Down)),
            (SlashMirror, Right) => res.push(self.move_in_dir(Up)),
            (SlashMirror, Up) => res.push(self.move_in_dir(Right)),
            (SlashMirror, Down) => res.push(self.move_in_dir(Left)),
            (BSlashMirror, Left) => res.push(self.move_in_dir(Up)),
            (BSlashMirror, Right) => res.push(self.move_in_dir(Down)),
            (BSlashMirror, Up) => res.push(self.move_in_dir(Left)),
            (BSlashMirror, Down) => res.push(self.move_in_dir(Right)),
            (VerticalSplitter, Left) => {
                res.push(self.move_in_dir(Up));
                res.push(self.move_in_dir(Down));
            }
            (VerticalSplitter, Right) => {
                res.push(self.move_in_dir(Up));
                res.push(self.move_in_dir(Down));
            }
            (VerticalSplitter, d) => res.push(self.move_in_dir(d)),
            (HorizontalSplitter, Left) => res.push(self.move_in_dir(Left)),
            (HorizontalSplitter, Right) => res.push(self.move_in_dir(Right)),
            (HorizontalSplitter, _) => {
                res.push(self.move_in_dir(Left));
                res.push(self.move_in_dir(Right));
            }
        }
        res
    }

    fn move_in_dir(&self, dir: Direction) -> Self {
        use Direction::*;
        let direction = dir;
        match dir {
            Left => Self {
                x: self.x.saturating_sub(1),
                y: self.y,
                direction,
            },
            Right => Self {
                x: self.x + 1,
                y: self.y,
                direction,
            },
            Up => Self {
                x: self.x,
                y: self.y.saturating_sub(1),
                direction,
            },
            Down => Self {
                x: self.x,
                y: self.y + 1,
                direction,
            },
        }
    }
}

fn value_from_start_pos(pos: Position, grid: &Vec<Vec<GridPosition>>) -> usize {
    let mut grid = (*grid).clone();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(pos);
    let mut visited = HashSet::new();
    while let Some(p) = to_visit.pop_back() {
        visited.insert(p);
        let grid_position = &mut grid[p.y][p.x];
        grid_position.energized = true;
        let next = p.next(&grid_position.element);
        next.iter().for_each(|p| {
            if !visited.contains(p) && p.x < grid[0].len() && p.y < grid.len() {
                visited.insert(*p);
                to_visit.push_back(*p);
            }
        });
    }
    let res: usize = grid
        .iter()
        .flat_map(|l| l.iter().filter(|p| p.energized))
        .count();
    res
}

fn part1(text: &str) -> Result<()> {
    let grid = parse_input(text)?;
    let res = value_from_start_pos(
        Position {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
        &grid,
    );
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    use Direction::*;
    let grid = parse_input(text)?;
    let start_positions = (0..grid.len())
        .map(|y| [(0, y, Right), (grid[0].len() - 1, y, Left)])
        .chain((0..grid[0].len()).map(|x| [(x, 0, Down), (x, grid.len() - 1, Up)]))
        .flatten();
    let res: usize = start_positions
        .map(|(x, y, direction)| value_from_start_pos(Position { x, y, direction }, &grid))
        .max()
        .context("expected value")?;
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(16).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
