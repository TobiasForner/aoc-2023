use anyhow::{bail, Context, Result};

use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn part1(text: &str) -> Result<()> {
    let res = 0;
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let res: usize = 0;
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day16.txt").expect("expected readable file");
    let _ = part1(&text);
    let _ = part2(&text);
}
