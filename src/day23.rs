use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use crate::util::{self, Direction};

type Position = (usize, usize);

/// stores shortcuts between junctions
#[derive(Debug)]
struct NavigationMap {
    start: Position,
    // maps a position to the potential movement options from that position
    navigation_options: HashMap<Position, HashMap<Direction, (Position, usize)>>,
}

impl NavigationMap {
    fn new(start: (usize, usize)) -> Self {
        NavigationMap {
            start,
            navigation_options: HashMap::new(),
        }
    }

    fn add_path(&mut self, start: (usize, usize), direction: &Direction, dest: (Position, usize)) {
        let lookup = self.navigation_options.entry(start).or_default();
        lookup.insert(direction.clone(), dest);
    }

    fn get_options_from(&self, start: Position) -> Vec<(Direction, (Position, usize))> {
        self.navigation_options
            .get(&start)
            .map(|c| {
                c.iter()
                    .map(|(d, ((y, x), steps))| (d.clone(), ((*y, *x), *steps)))
                    .collect()
            })
            .unwrap_or(vec![])
    }
}

/// all possible directions for the next step from `position`
fn possible_directions_from(
    position: (usize, usize),
    positions: &[Vec<char>],
    slopes_ok: bool,
) -> Vec<Direction> {
    use Direction::*;
    let (y, x) = position;
    if positions[y][x] == '#' {
        return vec![];
    }
    let mut directions = vec![];
    if slopes_ok {
        if x > 0 && positions[y][x - 1] != '#' {
            directions.push(Left);
        }
        if y > 0 && positions[y - 1][x] != '#' {
            directions.push(Up);
        }
        if x + 1 < positions[0].len() && positions[y][x + 1] != '#' {
            directions.push(Right);
        }
        if y + 1 < positions.len() && positions[y + 1][x] != '#' {
            directions.push(Down);
        }
    } else {
        if x > 0 && !['>', '#'].contains(&positions[y][x - 1]) {
            directions.push(Left);
        }
        if y > 0 && !['v', '#'].contains(&positions[y - 1][x]) {
            directions.push(Up);
        }
        if x + 1 < positions[0].len() && !['<', '#'].contains(&positions[y][x + 1]) {
            directions.push(Right);
        }
        if y + 1 < positions.len() && !['^', '#'].contains(&positions[y + 1][x]) {
            directions.push(Down);
        }
    }
    directions
}

/// a position is a junction if at least 3 adjacent positions are not forrest
fn position_is_junction(position: (usize, usize), positions: &[Vec<char>]) -> bool {
    let (y, x) = position;
    [
        (y.saturating_sub(1), x),
        ((y + 1).min(positions.len() - 1), x),
        (y, x.saturating_sub(1)),
        (y, (x + 1).min(positions[y].len() - 1)),
    ]
    .iter()
    .filter(|(y, x)| positions[*y][*x] != '#')
    .count()
        > 2
}

/// Walk one step from `position` in `direction`
fn walk(position: (usize, usize), direction: &Direction) -> (usize, usize) {
    use Direction::*;
    let (y, x) = position;
    match direction {
        Left => (y, x - 1),
        Up => (y - 1, x),
        Right => (y, x + 1),
        Down => (y + 1, x),
    }
}

/// start at `position` with initial and start walking, taking the first step in `direction`
/// Stops at the first junction
fn explore_from(
    position: (usize, usize),
    positions: &[Vec<char>],
    direction: &Direction,
    slopes_ok: bool,
) -> Option<(Position, usize)> {
    let mut last_pos = position;
    let mut position = position;
    println!("pos: {position:?}");
    if possible_directions_from(position, positions, slopes_ok).contains(direction) {
        position = walk(position, direction);
    } else {
        return None;
    }
    let mut steps = 1;
    while !position_is_junction(position, positions) {
        println!("pos: {position:?}");
        let next_directions = possible_directions_from(position, positions, slopes_ok);
        let next_positions: Vec<(usize, usize)> = next_directions
            .iter()
            .map(|d| walk(position, d))
            .filter(|p| *p != last_pos)
            .collect();
        println!("next: {next_positions:?}");
        if next_positions.is_empty() {
            return Some(((position.0, position.1), steps));
        } else {
            assert_eq!(next_positions.len(), 1);
            last_pos = position;
            position = next_positions[0];
            steps += 1;
        }
    }
    Some(((position.0, position.1), steps))
}

fn parse_input(text: &str, slopes_ok: bool) -> (NavigationMap, (usize, usize)) {
    let positions: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let start = (0, 1);
    let mut nav_map = NavigationMap::new(start);

    let dest = explore_from(start, &positions, &Direction::Down, slopes_ok).unwrap();
    nav_map.add_path(start, &Direction::Down, dest);

    let end = (positions.len() - 1, positions[0].len() - 2);

    positions.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            let position = (y, x);
            if position_is_junction(position, &positions) {
                possible_directions_from(position, &positions, slopes_ok)
                    .iter()
                    .for_each(|d| {
                        if let Some(dest) = explore_from(position, &positions, d, slopes_ok) {
                            nav_map.add_path(position, d, dest);
                        }
                    });
            }
        })
    });

    (nav_map, end)
}

/// Recursively computes the longest simple path from start to end
fn longest_path_to_end(
    start: (usize, usize),
    end: (usize, usize),
    nav_map: &NavigationMap,
    visited: HashSet<(usize, usize)>,
    steps: usize,
) -> Option<usize> {
    let mut visited = visited;
    visited.insert(start);

    nav_map
        .get_options_from(start)
        .iter()
        .filter_map(|(_, ((y, x), extra_steps))| {
            if visited.contains(&(*y, *x)) {
                None
            } else if (*y, *x) == end {
                Some(steps + extra_steps)
            } else {
                longest_path_to_end((*y, *x), end, nav_map, visited.clone(), steps + extra_steps)
            }
        })
        .max()
}

fn part2_comp(text: &str) -> Result<usize> {
    let (nav_map, end) = parse_input(text, true);
    let visited = HashSet::new();
    longest_path_to_end(nav_map.start, end, &nav_map, visited, 0).context("")
}
fn part1_comp(text: &str) -> Result<usize> {
    let (nav_map, end) = parse_input(text, false);
    let visited = HashSet::new();
    longest_path_to_end(nav_map.start, end, &nav_map, visited, 0).context("")
}

fn part1(text: &str) -> Result<()> {
    let res = part1_comp(text).unwrap();
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let res: usize = part2_comp(text).unwrap();
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(23).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}

#[test]
fn test_part1() {
    let text = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let res = part1_comp(text).unwrap();
    assert_eq!(res, 94);
}

#[test]
fn test_part2() {
    let text = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let res = part2_comp(text).unwrap();
    assert_eq!(res, 154);
}
