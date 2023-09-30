#![allow(dead_code)]
mod aoj;
mod atcoder;
mod nlp_100_knock;
mod project_euler;

#[allow(unused)]
use itertools::Itertools;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc3xx::abc322::a_first_abc_2::run3(8, String::from("ABABCABC")));
}
