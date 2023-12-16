use anyhow::Result;

use std::{collections::HashSet, fs};

fn parse_input(text: &str) -> Vec<Vec<char>> {
    text.lines().map(|l| l.chars().collect()).collect()
}

fn part1(text: &str) -> Result<()> {
    let mut input = parse_input(text);
    input = roll_north(&input);
    let res: usize = value(&input);
    println!("part 1: {res}");
    Ok(())
}

fn value(stones: &Vec<Vec<char>>) -> usize {
    let len = stones.len();
    let res: usize = stones
        .into_iter()
        .enumerate()
        .map(|(r, l)| {
            l.into_iter()
                .map(move |c| if *c == 'O' { len - r } else { 0 })
        })
        .flatten()
        .sum();
    res
}

fn roll_north(stones: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input = stones.clone();
    for col in 0..input[0].len() {
        let mut start = input.len();
        let mut end = input.len();
        let mut count = 0;
        while start > 0 {
            start -= 1;
            if input[start][col] == '#' {
                for i in start + 1..=start + count {
                    input[i][col] = 'O';
                }
                for i in (start + count + 1)..end {
                    input[i][col] = '.';
                }
                end = start;
                count = 0;
            }
            if input[start][col] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            for i in 0..count {
                input[i][col] = 'O';
            }
            for i in count..end {
                input[i][col] = '.';
            }
        }
    }
    input
}

fn roll_west(stones: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input = stones.clone();
    for row in 0..input.len() {
        let mut start = input[row].len();
        let mut end = input[row].len();
        let mut count = 0;
        while start > 0 {
            start -= 1;
            if input[row][start] == '#' {
                for i in start + 1..=start + count {
                    input[row][i] = 'O';
                }
                for i in (start + count + 1)..end {
                    input[row][i] = '.';
                }
                end = start;
                count = 0;
            }
            if input[row][start] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            for i in 0..count {
                input[row][i] = 'O';
            }
            for i in count..end {
                input[row][i] = '.';
            }
        }
    }
    input
}

fn roll_south(stones: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input = stones.clone();
    for col in 0..input[0].len() {
        let mut start = 0;
        let mut end = 0;
        let mut count = 0;
        while end < input.len() {
            end += 1;
            if input[end - 1][col] == '#' {
                let dot_count = end - start - count - 1;
                for i in start..(start + dot_count).min(end - 1) {
                    input[i][col] = '.';
                }
                for i in (start + dot_count)..(end - 1) {
                    input[i][col] = 'O';
                }
                start = end;
                count = 0;
            } else if input[end - 1][col] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            let dot_count = end - start - count;
            for i in start..start + dot_count {
                input[i][col] = '.';
            }
            for i in start + dot_count..end {
                input[i][col] = 'O';
            }
        }
    }
    input
}

fn roll_east(stones: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input = stones.clone();
    for row in 0..input.len() {
        let mut start = 0;
        let mut end = 0;
        let mut count = 0;
        while end < input[row].len() {
            end += 1;
            if input[row][end - 1] == '#' {
                let dot_count = end - start - count - 1;
                for i in start..start + dot_count {
                    input[row][i] = '.';
                }
                for i in (start + dot_count)..(end - 1) {
                    input[row][i] = 'O';
                }
                start = end;
                count = 0;
            }
            if input[row][end - 1] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            let dot_count = end - start - count;
            for i in start..start + dot_count {
                input[row][i] = '.';
            }
            for i in start + dot_count..end {
                input[row][i] = 'O';
            }
        }
    }
    input
}

fn part2(text: &str) -> Result<()> {
    let mut input = parse_input(text);

    let mut encountered = HashSet::new();

    let mut visits: Vec<Vec<Vec<char>>> = vec![];

    let mut cycles_left = 1000000000;

    while cycles_left > 0 {
        cycles_left -= 1;
        if encountered.contains(&input) {
            let index = visits.iter().position(|s| *s == input).unwrap();
            let cycle_len = visits.len() - index;
            let remainder = cycles_left % cycle_len;
            input = visits[index + remainder + 1].clone();
            break;
        }
        encountered.insert(input.clone());
        visits.push(input.clone());
        let res = roll_north(&input);
        let res = roll_west(&res);
        let res = roll_south(&res);
        let res = roll_east(&res);
        if res == input {
            break;
        }
        input = res;
    }
    let res: usize = value(&input);
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day14.txt").expect("expected readable file");
    let _ = part1(&text);
    let _ = part2(&text);
}
