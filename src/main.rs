mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day01::compute(),
        2 => day02::compute().expect("should not fail!"),
        3 => day03::compute(),
        4 => day04::compute(),
        5 => day05::compute(),
        6 => day06::compute(),
        7 => day07::compute(),
        8 => day08::compute(),
        9 => day09::compute(),
        10 => day10::compute(),
        11 => day11::compute(),
        12 => day12::compute(),
        13 => day13::compute(),
        14 => day14::compute(),
        15 => day15::compute(),
        16 => day16::compute(),
        _ => panic!("Invalid Day!"),
    }
}
