#![allow(dead_code)]
mod aoj;
mod atcoder;
mod nlp_100_knock;
mod project_euler;

fn main() {
    println!("Hello, world!");
    println!("{}", atcoder::abc1xx::abc146::b_rot_n::run2(2, String::from("ABCXYZ")));
}
