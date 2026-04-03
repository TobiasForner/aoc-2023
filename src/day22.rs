use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, Result};

use crate::util;

#[derive(Clone, Debug)]
enum Brick {
    /// x1, x2, y, z
    X(usize, usize, usize, usize),
    /// x, y1, y2, z
    Y(usize, usize, usize, usize),
    /// x, y, z1, z2
    Z(usize, usize, usize, usize),
}

impl Brick {
    fn lowest_x(&self) -> usize {
        match *self {
            Brick::X(x, _, _, _) => x,
            Brick::Y(x, _, _, _) => x,
            Brick::Z(x, _, _, _) => x,
        }
    }
    fn highest_x(&self) -> usize {
        match *self {
            Brick::X(_, x, _, _) => x,
            Brick::Y(x, _, _, _) => x,
            Brick::Z(x, _, _, _) => x,
        }
    }
    fn lowest_y(&self) -> usize {
        match *self {
            Brick::X(_, _, y, _) => y,
            Brick::Y(_, y, _, _) => y,
            Brick::Z(_, y, _, _) => y,
        }
    }
    fn highest_y(&self) -> usize {
        match *self {
            Brick::X(_, _, y, _) => y,
            Brick::Y(_, _, y, _) => y,
            Brick::Z(_, y, _, _) => y,
        }
    }
    fn lowest_z(&self) -> usize {
        match *self {
            Brick::X(_, _, _, z) => z,
            Brick::Y(_, _, _, z) => z,
            Brick::Z(_, _, z1, _) => z1,
        }
    }

    fn footprint(&self) -> Vec<(usize, usize)> {
        match *self {
            Brick::X(x1, x2, y, _) => (x1..=x2).map(|x| (x, y)).collect(),
            Brick::Y(x, y1, y2, _) => (y1..=y2).map(|y| (x, y)).collect(),
            Brick::Z(x, y, _, _) => vec![(x, y)],
        }
    }
    fn highest_z(&self) -> usize {
        match *self {
            Brick::X(_, _, _, z) => z,
            Brick::Y(_, _, _, z) => z,
            Brick::Z(_, _, _, z2) => z2,
        }
    }

    fn fall_to_height(&mut self, height: usize) {
        match self {
            Brick::X(_, _, _, z) => {
                *z = height;
            }
            Brick::Y(_, _, _, z) => {
                *z = height;
            }
            Brick::Z(_, _, z1, z2) => {
                let diff = *z2 - *z1;
                *z1 = height;
                *z2 = height + diff;
            }
        }
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Brick::*;
        let (start, end) = s.split_once('~').context("")?;
        let sc: Vec<usize> = start
            .split(',')
            .map(|c| c.parse().context(""))
            .collect::<Result<Vec<usize>>>()?;
        let ec: Vec<usize> = end
            .split(',')
            .map(|c| c.parse().context(""))
            .collect::<Result<Vec<usize>>>()?;
        Ok(if sc[0] != ec[0] {
            X(sc[0].min(ec[0]), sc[0].max(ec[0]), sc[1], sc[2])
        } else if sc[1] != ec[1] {
            Y(sc[0], sc[1].min(ec[1]), sc[1].max(ec[1]), sc[2])
        } else {
            Z(sc[0], sc[1], sc[2].min(ec[2]), sc[2].max(ec[2]))
        })
    }
}

struct HeightMap {
    map: Vec<Vec<usize>>,
    x_offset: usize,
    y_offset: usize,
    supports: HashMap<(usize, usize, usize), usize>,
}

impl HeightMap {
    fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        let map: Vec<Vec<usize>> = (x_min..=x_max)
            .map(|_| vec![0; y_max - y_min + 1])
            .collect();
        Self {
            map,
            x_offset: x_min,
            y_offset: y_min,
            supports: HashMap::new(),
        }
    }

    fn height_at(&self, x: usize, y: usize) -> usize {
        self.map[x - self.x_offset][y - self.y_offset]
    }

    fn set_height_at(&mut self, x: usize, y: usize, z: usize, brick_num: usize) {
        self.map[x - self.x_offset][y - self.y_offset] = z;
        self.supports.insert((x, y, z), brick_num);
    }
}

fn parse_input(text: &str) -> Result<Vec<Brick>> {
    text.lines().map(|l| l.parse().context("")).collect()
}

fn compute_steady_state(bricks: &[Brick]) -> (Vec<Brick>, HeightMap, usize) {
    let mut bricks = bricks.to_vec();
    // start with everything zero
    let x_min = bricks.iter().map(|b| b.lowest_x()).min().unwrap();
    let x_max = bricks.iter().map(|b| b.highest_x()).max().unwrap();
    let y_min = bricks.iter().map(|b| b.lowest_y()).min().unwrap();
    let y_max = bricks.iter().map(|b| b.highest_y()).max().unwrap();
    // process bricks in order of increasing lower z coordinate
    bricks.sort_by_key(|b| b.lowest_z());
    let mut height_map = HeightMap::new(x_min, x_max, y_min, y_max);
    let mut falls_count = 0;
    bricks.iter_mut().enumerate().for_each(|(bid, b)| {
        let footprint = b.footprint();
        let max_height_under_brick = footprint
            .iter()
            .map(|(x, y)| height_map.height_at(*x, *y))
            .max()
            .unwrap();
        let z_min = b.lowest_z();
        let new_z = max_height_under_brick + 1;
        if z_min > new_z {
            falls_count += 1;
            b.fall_to_height(new_z);
            let top = b.highest_z();
            footprint.iter().for_each(|(x, y)| {
                height_map.set_height_at(*x, *y, top, bid);
            });
        } else {
            let top = b.highest_z();
            footprint.iter().for_each(|(x, y)| {
                height_map.set_height_at(*x, *y, top, bid);
            });
        }
    });
    (bricks, height_map, falls_count)
}

fn part1(text: &str) -> Result<()> {
    let bricks = parse_input(text)?;
    let (bricks, height_map, _) = compute_steady_state(&bricks);
    let mut required_bricks: HashSet<usize> = HashSet::new();
    let mut res2 = 0;
    bricks.iter().for_each(|b| {
        let bot = b.lowest_z();
        if bot > 0 {
            let support: HashSet<_> = b
                .footprint()
                .iter()
                .filter_map(|(x, y)| height_map.supports.get(&(*x, *y, bot - 1)))
                .collect();
            if support.len() == 1 {
                required_bricks.insert(**support.iter().next().unwrap());
                res2 += 1;
            }
        }
    });

    let res = bricks.len() - required_bricks.len();
    println!("part 1: {res}");
    Ok(())
}
fn part2(text: &str) -> Result<()> {
    let res = part2_comp(text)?;
    println!("part 2: {res}");
    Ok(())
}

fn part2_comp(text: &str) -> Result<usize> {
    let bricks = parse_input(text)?;
    let (bricks, _, _) = compute_steady_state(&bricks);

    let mut res: usize = 0;
    bricks.iter().enumerate().for_each(|(pos, _)| {
        let mut rem_bricks = bricks.clone();
        rem_bricks.remove(pos);
        let (_, _, falls) = compute_steady_state(&rem_bricks);
        res += falls;
    });
    Ok(res)
}

pub fn compute() {
    let text = util::read_input_file(22).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}

#[test]
fn test_part2() {
    let text = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let res = part2_comp(text).unwrap();
    assert_eq!(res, 7);
}
