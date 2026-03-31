use anyhow::Result;

use crate::util;

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
    let text = util::read_input_file(25).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
