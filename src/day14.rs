use anyhow::Result;

use std::collections::HashSet;

use crate::util;

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

fn value(stones: &[Vec<char>]) -> usize {
    let len = stones.len();
    let res: usize = stones
        .iter()
        .enumerate()
        .flat_map(|(r, l)| l.iter().map(move |c| if *c == 'O' { len - r } else { 0 }))
        .sum();
    res
}

fn roll_north(stones: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut input = stones.to_owned();
    for col in 0..input[0].len() {
        let mut start = input.len();
        let mut end = input.len();
        let mut count = 0;
        while start > 0 {
            start -= 1;
            if input[start][col] == '#' {
                (start + 1..=start + count).for_each(|i| {
                    input[i][col] = 'O';
                });
                ((start + count + 1)..end).for_each(|i| {
                    input[i][col] = '.';
                });
                end = start;
                count = 0;
            }
            if input[start][col] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            input.iter_mut().take(count).for_each(|row| {
                row[col] = 'O';
            });
            (count..end).for_each(|i| {
                input[i][col] = '.';
            })
        }
    }
    input
}

fn roll_west(stones: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut input = stones.to_vec();
    input.iter_mut().for_each(|row| {
        let mut start = row.len();
        let mut end = row.len();
        let mut count = 0;
        while start > 0 {
            start -= 1;
            if row[start] == '#' {
                (start + 1..=start + count).for_each(|i| {
                    row[i] = 'O';
                });
                ((start + count + 1)..end).for_each(|i| {
                    row[i] = '.';
                });
                end = start;
                count = 0;
            }
            if row[start] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            row.iter_mut().take(count).for_each(|x| *x = 'O');
            row.iter_mut().take(end).skip(count).for_each(|x| *x = '.');
        }
    });
    input.to_vec()
}

fn roll_south(stones: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut input = stones.to_vec();
    for col in 0..input[0].len() {
        let mut start = 0;
        let mut end = 0;
        let mut count = 0;
        while end < input.len() {
            end += 1;
            if input[end - 1][col] == '#' {
                let dot_count = end - start - count - 1;
                input[start..(start + dot_count).min(end - 1)]
                    .iter_mut()
                    .for_each(|row| {
                        row[col] = '.';
                    });
                ((start + dot_count)..(end - 1)).for_each(|i| {
                    input[i][col] = 'O';
                });
                start = end;
                count = 0;
            } else if input[end - 1][col] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            let dot_count = end - start - count;
            input[start..start + dot_count].iter_mut().for_each(|row| {
                row[col] = '.';
            });
            input[start + dot_count..end].iter_mut().for_each(|row| {
                row[col] = 'O';
            });
        }
    }
    input
}

fn roll_east(stones: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut input = stones.to_vec();
    input.iter_mut().for_each(|row| {
        let mut start = 0;
        let mut end = 0;
        let mut count = 0;
        while end < row.len() {
            end += 1;
            if row[end - 1] == '#' {
                let dot_count = end - start - count - 1;
                (start..start + dot_count).for_each(|i| {
                    row[i] = '.';
                });
                ((start + dot_count)..(end - 1)).for_each(|i| {
                    row[i] = 'O';
                });
                start = end;
                count = 0;
            }
            if row[end - 1] == 'O' {
                count += 1;
            }
        }
        if count > 0 {
            let dot_count = end - start - count;
            (start..start + dot_count).for_each(|i| {
                row[i] = '.';
            });
            (start + dot_count..end).for_each(|i| {
                row[i] = 'O';
            });
        }
    });
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
    let text = util::read_input_file(14).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
