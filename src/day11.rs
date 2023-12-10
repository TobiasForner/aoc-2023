use std::{collections::HashSet, fs};

fn parse_input(text: &str) -> Vec<(usize, usize)> {
    text.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn dist_sums(text: &str, aging: usize) -> usize {
    let positions = parse_input(text);
    let mut res: usize = 0;
    let used_x: HashSet<usize> = positions.iter().map(|(x, _)| *x).collect();
    let used_y: HashSet<usize> = positions.iter().map(|(_, y)| *y).collect();
    let l = positions.len();
    (0..l).for_each(|i1| {
        (i1 + 1..l).for_each(|i2| {
            let (x1, y1) = positions[i1];
            let (x2, y2) = positions[i2];
            let x_count: usize = (x1.min(x2)..x1.max(x2))
                .map(|x| if used_x.contains(&x) { 1 } else { aging })
                .sum();
            let y_count: usize = (y1.min(y2)..y1.max(y2))
                .map(|y| if used_y.contains(&y) { 1 } else { aging })
                .sum();
            res += x_count + y_count;
        })
    });
    res
}

fn part1(text: &str) {
    let res = dist_sums(text, 2);
    println!("part 1: {res}");
}

fn part2(text: &str) {
    let res = dist_sums(text, 1000000);

    println!("part 2: {res}");
}

pub fn compute() {
    let text = fs::read_to_string("inputs/day11.txt").expect("expected readable file");
    part1(&text);

    part2(&text);
}
