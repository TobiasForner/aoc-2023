use anyhow::Result;

use crate::util;

fn parse_input(text: &str) -> Result<Vec<(u64, u64)>> {
    let vecs = text
        .lines()
        .map(|l| {
            let mut nums = l.split_ascii_whitespace();
            nums.next();
            nums.map(|n| n.parse().unwrap()).collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let res = vecs[0].clone().into_iter().zip(vecs[1].clone()).collect();
    Ok(res)
}

fn part1(text: &str) {
    let input = parse_input(text).unwrap();
    let res = input
        .iter()
        .map(|(t, d)| (1..=t - 1).filter(|p| is_winning(*p, *t, *d)).count())
        .reduce(|acc, c| acc * c)
        .unwrap();
    println!("part 1: {res}");
}

fn lowest_winning(start: u64, end: u64, time: u64, dist: u64) -> Option<u64> {
    let mid = (start + end).div_ceil(2);
    //println!("{start}; {end}; {mid}");
    if is_winning(mid, time, dist) {
        if start >= end {
            return Some(mid);
        }
        if start == end - 1 {
            if is_winning(start, time, dist) {
                return Some(start);
            } else {
                return Some(mid);
            }
        }
        lowest_winning(start, mid, time, dist)
    } else {
        if start >= end {
            return None;
        }
        let lower = lowest_winning(start, mid.saturating_sub(1), time, dist);
        if lower.is_some() {
            return lower;
        }
        lowest_winning(mid + 1, end, time, dist)
    }
}

fn is_winning(press_time: u64, time: u64, dist: u64) -> bool {
    (time - press_time) * press_time > dist
}
fn largest_winning(start: u64, end: u64, time: u64, dist: u64) -> Option<u64> {
    let mid = (start + end).div_ceil(2);
    //println!("{start}; {end}; {mid}");
    if is_winning(mid, time, dist) {
        if start >= end {
            return Some(mid);
        }
        largest_winning(mid, end, time, dist)
    } else {
        if start >= end {
            return None;
        }
        let upper = largest_winning(mid + 1, end, time, dist);
        if upper.is_some() {
            return upper;
        }
        largest_winning(start, mid.saturating_sub(1), time, dist)
    }
}

fn part2(text: &str) {
    let numbers: Vec<u64> = text
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect();
    println!("{numbers:?}");
    let lowest = lowest_winning(1, numbers[0] - 1, numbers[0], numbers[1]);
    let largest = largest_winning(1, numbers[0] - 1, numbers[0], numbers[1]);
    if let (Some(n), Some(m)) = (lowest, largest) {
        let res = m - n + 1;
        println!("part 2: {res}");
    }
    println!("{lowest:?}; {largest:?}");
}

pub fn compute() {
    let text = util::read_input_file(6).unwrap();
    part1(&text);
    part2(&text);
}
