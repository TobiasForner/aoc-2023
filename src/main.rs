mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use clap::Parser;

///rtnertn
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day01::compute(),
        2 => day02::compute(),
        3 => day03::compute(),
        4 => day04::compute(),
        5 => day05::compute(),
        6 => day06::compute(),
        7 => day07::compute(),
        _ => panic!("Invalid Day!"),
    }
    //day01::compute();
    //day02::compute();
    //day03::compute();
    //day04::compute();
    //day05::compute();
    //day06::compute();
    day07::compute();
}
