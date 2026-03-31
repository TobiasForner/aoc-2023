use anyhow::{bail, Result};
use std::ops::IndexMut;

use crate::util;

fn parse_input(text: &str) -> Result<Vec<Vec<i64>>> {
    text.lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i64>().or_else(|_| bail!("")))
                .collect()
        })
        .collect()
}

fn extrapolate(v: Vec<i64>) -> i64 {
    let mut lines = vec![v];
    while !lines[lines.len() - 1].iter().all(|n| *n == 0) {
        lines.push(
            lines
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect(),
        )
    }
    lines.last_mut().unwrap().push(0);
    for i in 2..=lines.len() {
        let i = lines.len() - i;
        let x = *lines[i + 1].last().unwrap();
        let y = *lines[i].last().unwrap();
        lines.index_mut(i).push(x + y);
    }
    *lines[0].last().unwrap()
}

fn extrapolate_backwards(v: Vec<i64>) -> i64 {
    let mut lines = vec![v];
    while !lines[lines.len() - 1].iter().all(|n| *n == 0) {
        lines.push(
            lines
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect(),
        )
    }
    lines.last_mut().unwrap().insert(0, 0);
    for i in 2..=lines.len() {
        let i = lines.len() - i;
        let x = lines[i + 1][0];
        let y = lines[i][0];
        lines.index_mut(i).insert(0, y - x);
    }
    lines[0][0]
}

fn part1(text: &str) -> Result<()> {
    let input = parse_input(text)?;
    let res: i64 = input.iter().map(|v| extrapolate(v.clone())).sum();
    println!("part 1: {res}");
    Ok(())
}

fn part2(text: &str) -> Result<()> {
    let input = parse_input(text)?;
    let res: i64 = input.iter().map(|v| extrapolate_backwards(v.clone())).sum();
    println!("part 1: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(9).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
