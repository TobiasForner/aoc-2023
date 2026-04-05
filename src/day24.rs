use std::str::FromStr;

use anyhow::{Context, Result};
use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;

use crate::util;

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    v_x: f64,
    v_y: f64,
    v_z: f64,
}

impl Hailstone {
    fn get_collision_times(&self, other: &Hailstone) -> Option<(f64, f64, f64, f64)> {
        let tmp = self.y + (other.x - self.x) / self.v_x * self.v_y - other.y;
        let t2 = tmp / (other.v_y * (1_f64 - (other.v_x * self.v_y / (self.v_x * other.v_y))));
        let t1 = (other.x + t2 * other.v_x - self.x) / self.v_x;
        let x = self.x + self.v_x * t1;
        let y = self.y + self.v_y * t1;
        Some((t1, t2, x, y))
    }
}

impl FromStr for Hailstone {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let coordinates: Vec<f64> = pos
            .split(", ")
            .map(|v| v.parse::<f64>().context(""))
            .collect::<Result<Vec<f64>>>()?;
        let velocity: Vec<f64> = vel
            .split(", ")
            .map(|v| v.parse().context(""))
            .collect::<Result<Vec<f64>>>()?;
        Ok(Self {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
            v_x: velocity[0],
            v_y: velocity[1],
            v_z: velocity[2],
        })
    }
}

fn part1(text: &str) -> Result<()> {
    let hailstones: Vec<Hailstone> = util::parse_from_text_lines(text)?;

    let res = hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, h1)| {
            hailstones.iter().take(i + 1).filter_map(|h2| {
                h1.get_collision_times(h2).and_then(|(t1, t2, x, y)| {
                    if t1.is_nan()
                        || t2.is_nan()
                        || t1 < 0.0
                        || t2 < 0.0
                        || !(200000000000000.0..400000000000000.0).contains(&x)
                        || !(200000000000000.0..400000000000000.0).contains(&y)
                    {
                        None
                    } else {
                        Some((t1, t2))
                    }
                })
            })
        })
        .count();
    println!("part 1: {res}");
    Ok(())
}

/// returns a 3-dim vector, each dim contains the coefficients in a row
/// each row has the order x,y,z,vx,vy,vz, const
fn cross_coefficients(x_other: Vec<f64>, v_other: Vec<f64>) -> Vec<Vec<f64>> {
    let row0 = vec![
        0.0,
        v_other[2],
        -v_other[1],
        0.0,
        -x_other[2],
        x_other[1],
        -x_other[1] * v_other[2] + x_other[2] * v_other[1],
    ];

    let row1 = vec![
        -v_other[2],
        0.0,
        v_other[0],
        x_other[2],
        0.0,
        -x_other[0],
        -x_other[2] * v_other[0] + x_other[0] * v_other[2],
    ];

    let row2 = vec![
        v_other[1],
        -v_other[0],
        0.0,
        -x_other[1],
        x_other[0],
        0.0,
        -x_other[0] * v_other[1] + x_other[1] * v_other[0],
    ];

    vec![row0, row1, row2]
}

fn hailstone_cross_coefficients(h: &Hailstone) -> Vec<Vec<f64>> {
    cross_coefficients(vec![h.x, h.y, h.z], vec![h.v_x, h.v_y, h.v_z])
}

fn part2(text: &str) -> Result<()> {
    let hailstones: Vec<Hailstone> = util::parse_from_text_lines(text)?;
    let h0 = &hailstones[0];
    let h1 = &hailstones[1];
    let h2 = &hailstones[2];
    let h3 = &hailstones[3];

    let c0 = hailstone_cross_coefficients(h0);
    let c1 = hailstone_cross_coefficients(h1);
    let c2 = hailstone_cross_coefficients(h2);
    let c3 = hailstone_cross_coefficients(h3);
    let mut a: Array2<f64> = Array2::from_elem((6, 6), 0.0);
    let mut b: Array1<f64> = Array1::zeros(6);
    (0..3).for_each(|row| {
        (0..6).for_each(|i| {
            a[[row, i]] += c0[row][i];
            a[[row, i]] -= c1[row][i];
        });
        b[row] -= c0[row][6];
        b[row] += c1[row][6];
    });
    (3..6).for_each(|row| {
        (0..6).for_each(|i| {
            a[[row, i]] += c2[row - 3][i];
            a[[row, i]] -= c3[row - 3][i];
        });
        b[row] -= c2[row - 3][6];
        b[row] += c3[row - 3][6];
    });
    let x = a.solve_into(b).unwrap();
    let res = (x[0] + x[1] + x[2]) as i64;

    println!("part 2: {res}");
    Ok(())
}

pub fn compute() {
    let text = util::read_input_file(24).unwrap();
    let _ = part1(&text);
    let _ = part2(&text);
}
