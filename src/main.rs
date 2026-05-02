#![allow(dead_code)]
// mod aoj;
mod atcoder;
mod paiza;
// mod dp;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
mod yukicoder;
mod other;

fn main() {
    println!("{:?}", atcoder::abc2xx::abc264::e::run(5, 5, 10, vec![ (2, 3), (4, 10), (5, 10), (6, 9), (2, 9), (4, 8), (1, 7), (3, 6), (8, 10), (1, 8)], 6, vec![3, 5, 8, 10, 2, 7]));
}
