#![allow(dead_code)]
mod atcoder;
mod nlp_100_knock;
mod aoj;

fn main() {
    println!("Hello, world!");
    println!("{}", atcoder::abc0xx::abc030::a_winning_percentage_calculation::run(5.0, 2.0, 6.0, 3.0))
}
