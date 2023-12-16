use std::fs;

fn map_numbers(s: String, ignore_words: bool) -> Vec<u32> {
    let mut res: Vec<(usize, u32)> = s
        .chars()
        .enumerate()
        .filter_map(|c| {
            if c.1.is_ascii_digit() {
                Some((c.0, c.1.to_digit(10).expect("msg")))
            } else {
                None
            }
        })
        .collect();
    if !ignore_words {
        let nums = vec![
            ("one", 1 as u32),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        let mut ind: Vec<(usize, u32)> = nums
            .iter()
            .map(|n| s.match_indices(n.0).map(|m| (m.0, n.1)))
            .flatten()
            .collect();

        res.append(&mut ind);
    }
    res.sort();
    res.iter().map(|t| t.1).collect()
}

fn value(text: &str, ignore_words: bool) -> u32 {
    let parsed_lines: Vec<Vec<u32>> = text
        .lines()
        .map(|l| map_numbers(l.to_string(), ignore_words))
        .collect();
    let vals: Vec<u32> = parsed_lines
        .into_iter()
        .map(|l| {
            let v = l[0];
            let v2 = l.last().expect("msg");
            10 * v + v2
        })
        .collect();
    let res: u32 = vals.iter().sum();
    res
}

fn part1(text: &str) {
    let res = value(&text, true);

    println!("{res}")
}

fn part2(text: &str) {
    let res = value(&text, false);

    println!("{res}")
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day01.txt").expect("");

    part1(&text);
    part2(&text);
}
