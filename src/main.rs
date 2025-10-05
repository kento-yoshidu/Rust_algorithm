#![allow(dead_code)]
// mod aoj;
mod atcoder;
mod paiza;
// mod dp;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
// mod yukicoder;

fn main() {
    atcoder::abc2xx::abc226::c::run(3, vec![(3, 0, None), (5, 1, Some(vec![1])), (7, 1, Some(vec![1]))]);
    // atcoder::abc0xx::abc001::c::run(1462, 628);
    // println!("{:?}", atcoder::abc0xx::abc001::c::run(161, 8));
    // println!("Hello World");
}
