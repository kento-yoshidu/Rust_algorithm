#![allow(dead_code)]
mod atcoder;
mod nlp_100_knock;
mod aoj;

fn main() {
    println!("Hello, world!");
    println!("{}", atcoder::abc1xx::abc182::b_almost_gcd::run(5, vec![8, 9, 18, 90, 72]));
}
