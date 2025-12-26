use anyhow::{bail, Context, Result};

use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn part1(text: &str) -> Result<()> {
    let map: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let mut start_x = 0;
    let mut start_y = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                start_x = x;
                start_y = y;
            }
        }
    }

    let mut current: HashSet<(usize, usize)> = HashSet::new();
    current.insert((start_x, start_y));
    for _ in 0..64 {
        let next = current
            .iter()
            .map(|(x, y)| [(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)])
            .flatten()
            .filter(|(x, y)| {
                *y < map.len() && *x < map[*y].len() && (map[*y][*x] == '.' || map[*y][*x] == 'S')
            })
            .collect();
        current = next;
    }
    let res = current.len();
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let res: usize = 0;
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day21.txt").expect("expected readable file");
    let _ = part1(&text);
    let _ = part2(&text);
}
