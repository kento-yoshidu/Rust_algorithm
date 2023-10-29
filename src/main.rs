#![allow(dead_code)]
// mod aoj;
mod atcoder;
mod basic;
// mod nlp_100_knock;
// mod project_euler;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc3xx::abc326::a_2up_3down::run(1, 4));
    println!("{}", atcoder::abc3xx::abc326::a_2up_3down::run(99, 96));
    println!("{}", atcoder::abc3xx::abc326::a_2up_3down::run(100, 1));
}
