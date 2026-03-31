use anyhow::{Context, Result, bail};
use std::collections::{HashSet, VecDeque};

use crate::util;

fn exits(p: &(usize, usize), map: &[Vec<char>]) -> Result<HashSet<(usize, usize)>> {
    let mut res = HashSet::new();
    let y = p.1;
    let x = p.0;
    match map[y][x] {
        '|' => {
            res.insert((x, y + 1));
            res.insert((x, y - 1));
        }
        '-' => {
            res.insert((x - 1, y));
            res.insert((x + 1, y));
        }
        'L' => {
            res.insert((x, y - 1));
            res.insert((x + 1, y));
        }
        'J' => {
            res.insert((x - 1, y));
            res.insert((x, y - 1));
        }
        '7' => {
            res.insert((x - 1, y));
            res.insert((x, y + 1));
        }
        'F' => {
            res.insert((x + 1, y));
            res.insert((x, y + 1));
        }
        '.' => {}
        'S' => {
            res.insert((x, y + 1));
            res.insert((x, y - 1));
            res.insert((x - 1, y));
            res.insert((x + 1, y));
        }
        c => bail!("invalid char: {c}"),
    }
    Ok(res)
}

fn next_position(positions: &Vec<(usize, usize)>, map: &[Vec<char>]) -> Result<(usize, usize)> {
    let mut next_cands = exits(positions.last().context("")?, map)?;
    next_cands.remove(positions.last().context("positions should not be empty")?);
    next_cands.remove(&positions[positions.len() - 2]);
    next_cands
        .iter()
        .next()
        .context(format!("No next position for {positions:?}!"))
        .copied()
}

fn part1(text: &str) -> Result<()> {
    let map: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();

    let mut positions1 = vec![];
    map.iter().enumerate().for_each(|(y, v)| {
        if let Some((x, _)) = v.iter().enumerate().find(|(_, c)| **c == 'S') {
            positions1.push((x, y));
        }
    });
    let next: Vec<(usize, usize)> = exits(&positions1[0], &map)?
        .into_iter()
        .filter(|p| exits(p, &map).unwrap().contains(&positions1[0]))
        .collect();
    positions1.push(next[0]);
    let mut positions2 = vec![positions1[0], next[1]];
    while positions1[positions1.len() - 1] != positions2[positions2.len() - 1] {
        let p1 = next_position(&positions1, &map);
        positions1.push(p1?);
        let p2 = next_position(&positions2, &map);
        positions2.push(p2?);
    }

    let res: usize = positions1.len() - 1;
    println!("part 1: {res}");
    Ok(())
}

fn reached_by(pos: (usize, usize), map: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let x = pos.0;
    let y = pos.1;
    let mut res = HashSet::new();
    let nx = x.saturating_sub(1);

    if map[y][nx] == '.' || map[y][nx] == 'X' {
        res.insert((nx, y));
    }
    let nx = x.saturating_add(1).min(map[0].len() - 1);
    if map[y][nx] == '.' || map[y][nx] == 'X' {
        res.insert((nx, y));
    }
    let ny = y.saturating_sub(1);
    if map[ny][x] == '.' || map[ny][x] == 'X' {
        res.insert((x, ny));
    }
    let ny = y.saturating_add(1).min(map.len() - 1);
    if map[ny][x] == '.' || map[ny][x] == 'X' {
        res.insert((x, ny));
    }
    res
}

fn part2(text: &str) -> Result<()> {
    let mut map: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();

    let mut positions = vec![];
    map.iter().enumerate().for_each(|(y, v)| {
        if let Some((x, _)) = v.iter().enumerate().find(|(_, c)| **c == 'S') {
            positions.push((x, y));
        }
    });
    let next: Vec<(usize, usize)> = exits(&positions[0], &map)?
        .into_iter()
        .filter(|p| exits(p, &map).unwrap().contains(&positions[0]))
        .collect();
    positions.push(next[0]);
    while positions[positions.len() - 1] != positions[0] {
        let p = next_position(&positions, &map);
        positions.push(p?);
    }
    positions.remove(positions.len() - 1);

    let sx = positions[0].0;
    let sy = positions[0].1;
    let next = next.into_iter().collect::<HashSet<(usize, usize)>>();
    for c in ['|', '-', 'L', 'J', '7', 'F'] {
        map[sy][sx] = c;
        let next_cands = exits(&(sx, sy), &map)?;
        if next == next_cands {
            break;
        }
    }

    let loop_positions: HashSet<(usize, usize)> = positions.into_iter().collect();
    map = map
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, c)| {
                    if loop_positions.contains(&(x, y)) {
                        *c
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();

    let width = map[0].len();
    let add_vert: HashSet<(char, char)> = [
        ('|', 'J'),
        ('|', '|'),
        ('|', 'L'),
        ('7', 'J'),
        ('7', '|'),
        ('7', 'L'),
        ('F', 'J'),
        ('F', '|'),
        ('F', 'L'),
    ]
    .into_iter()
    .collect();
    let mut pos = 1;
    while pos < map.len() {
        let line = (0..width)
            .map(|x| {
                if add_vert.contains(&(map[pos - 1][x], map[pos][x])) {
                    '|'
                } else {
                    'X'
                }
            })
            .collect();
        map.insert(pos, line);
        pos += 2;
    }

    let add_hor: HashSet<(char, char)> = [
        ('-', '-'),
        ('-', 'J'),
        ('-', '7'),
        ('L', 'J'),
        ('L', '-'),
        ('L', '7'),
        ('F', 'J'),
        ('F', '-'),
        ('F', '7'),
    ]
    .into_iter()
    .collect();
    for line in map.iter_mut() {
        pos = 1;
        while pos < line.len() {
            let c = if add_hor.contains(&(line[pos - 1], line[pos])) {
                '-'
            } else {
                'X'
            };
            line.insert(pos, c);
            pos += 2;
        }
    }
    //print_map(&map);

    let mut res: usize = 0;

    let mut todo: VecDeque<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == '.')
                .map(move |(x, _)| (x, y))
        })
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
            let next = reached_by(pos, &map);
            let new = next.difference(&visited);
            new.for_each(|e| to_ex.push_back(*e));
            visited.insert(pos);
            new_visited.insert(pos);
        }
        if !new_visited
            .iter()
            .any(|(x, y)| *x == 0 || *x == map[0].len() - 1 || *y == 0 || *y == map.len() - 1)
        {
            let dots = new_visited
                .iter()
                .filter(|(x, y)| map[*y][*x] == '.')
                .count();
            res += dots;
        }
    }
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(10).unwrap();
    let res1 = part1(&text);
    if res1.is_err() {
        println!("{res1:?}");
    }
    let _ = part2(&text);
}
