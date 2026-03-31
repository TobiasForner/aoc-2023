use anyhow::{Context, Result};
use itertools::Itertools;

use crate::util;

fn parse_input(text: &str) -> Vec<Vec<u32>> {
    text.split(',')
        .map(|s| s.chars().map(|c| c as u32).collect())
        .collect()
}

fn part1(text: &str) -> Result<()> {
    let sequences = parse_input(text);

    let res: u32 = sequences
        .into_iter()
        .map(|s| {
            let mut current_val = 0;
            s.into_iter().for_each(|c| {
                current_val += c;
                current_val = (current_val * 17) % 256;
            });
            current_val
        })
        .sum();
    println!("part 1: {res}");
    Ok(())
}

fn hash_alg(s: &str) -> u32 {
    let mut current_val = 0;
    s.chars().for_each(|c| {
        current_val += c as u32;
        current_val = (current_val * 17) % 256;
    });
    current_val
}

struct Lens {
    label: String,
    focal_length: u32,
}

fn part2(text: &str) -> Result<()> {
    let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect();

    for sequence in text.split(',') {
        if let Some((label, _)) = sequence.split_once('-') {
            let box_num = hash_alg(label) as usize;
            let lbox = boxes.get_mut(box_num).context("")?;
            if let Some((p, _)) = lbox.iter().find_position(|l| l.label == label) {
                lbox.remove(p);
            }
        } else if let Some((label, focal_length)) = sequence.split_once('=') {
            let box_num = hash_alg(label) as usize;
            let lbox = boxes.get_mut(box_num).context("")?;
            let lens = Lens {
                label: label.to_string(),
                focal_length: focal_length.parse()?,
            };
            if let Some((p, _)) = lbox.iter().find_position(|l| l.label == label) {
                lbox[p] = lens;
            } else {
                lbox.push(lens);
            }
        }
    }
    let res: usize = boxes
        .into_iter()
        .enumerate()
        .flat_map(|(num, lenses)| {
            let box_num = num + 1;
            lenses
                .into_iter()
                .enumerate()
                .map(move |(ln, lens)| box_num * (ln + 1) * (lens.focal_length as usize))
        })
        .sum();
    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(15).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
