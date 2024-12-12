#![warn(clippy::pedantic)]

use std::time::Instant;

mod days;

fn main() {
    let start = Instant::now();

    let result = days::day1::part_one_reddit();

    let duration = start.elapsed();

    println!("{result} in {duration:?}");
}