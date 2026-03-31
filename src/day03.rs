use logos::Logos;
use std::collections::{HashMap, HashSet};

use crate::util;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[.]")]
enum Token {
    // Or regular expressions.
    #[regex("[0-9]+", |lex|lex.slice().parse().ok())]
    Number(u32),
    #[regex(r"[*+/&=\-@#%$]", |lex|lex.slice().chars().next().unwrap())]
    Special(char),
}

fn surrounding_positions(line: i64, start: i64, end: i64) -> impl Iterator<Item = (i64, i64)> {
    let around = start - 1..=end;
    let left_right = [(line, start - 1), (line, end)];
    return around
        .clone()
        .map(move |x| (line - 1, x))
        .chain(left_right.into_iter())
        .chain(around.map(move |x| (line + 1, x)));
}

fn part1(text: &str) {
    let lines: Vec<&str> = text.lines().collect();

    let mut numbers: Vec<(i64, i64, i64, u32)> = vec![];
    let mut specials = HashSet::new();

    lines.iter().enumerate().for_each(|(i, l)| {
        let lex = Token::lexer(l);
        lex.spanned().for_each(|(t, s)| {
            match t {
                Ok(Token::Number(n)) => {
                    numbers.push((i as i64, s.start as i64, s.end as i64, n));
                }
                Ok(Token::Special(_)) => {
                    specials.insert((i as i64, s.start as i64));
                }
                Err(()) => panic!("f"),
            };
        })
    });

    let res: u32 = numbers
        .iter()
        .map(|(line, start, end, num)| {
            if surrounding_positions(*line, *start, *end).any(|p| specials.contains(&p)) {
                *num
            } else {
                0
            }
        })
        .sum();
    println!("{res}");
}

fn part2(text: &str) {
    let lines: Vec<&str> = text.lines().collect();

    let mut numbers: Vec<(i64, i64, i64, u32)> = vec![];
    let mut specials: HashMap<(i64, i64), Vec<u32>> = HashMap::new();

    lines.iter().enumerate().for_each(|(i, l)| {
        let lex = Token::lexer(l);
        lex.spanned().for_each(|(t, s)| {
            match t {
                Ok(Token::Number(n)) => {
                    numbers.push((i as i64, s.start as i64, s.end as i64, n));
                }
                Ok(Token::Special(sp)) => {
                    if sp == '*' {
                        specials.insert((i as i64, s.start as i64), vec![]);
                    }
                }
                Err(()) => panic!("f"),
            };
        })
    });

    numbers.iter().for_each(|(l, s, e, n)| {
        surrounding_positions(*l, *s, *e).for_each(|(s, e)| {
            if let Some(v) = specials.get_mut(&(s, e)) {
                v.push(*n);
            }
        })
    });

    let res: u32 = specials
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();
    println!("{res}");
}

pub fn compute() {
    let text = util::read_input_file(3).unwrap();
    part1(&text);
    part2(&text);
}
