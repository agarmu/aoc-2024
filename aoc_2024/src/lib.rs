#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod day6;
#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;
extern crate pretty_assertions;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
aoc_lib! { year = 2024 }
