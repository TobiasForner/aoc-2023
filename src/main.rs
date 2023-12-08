mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

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
        _ => panic!("Invalid Day!"),
    }
}
