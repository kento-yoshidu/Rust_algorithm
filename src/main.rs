#![allow(dead_code)]

mod aoj;
mod atcoder;
// mod paiza;
// mod dp;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
// mod yukicoder;
// mod other;

fn main() {
    println!("{}", atcoder::other_contest::tdpc::a::run(3, vec![2, 3, 5]));
    println!("{}", atcoder::other_contest::tdpc::a::run(10, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
}
