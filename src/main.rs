#![allow(dead_code)]
mod aoj;
mod atcoder;
mod nlp_100_knock;
mod project_euler;

#[allow(unused)]
use itertools::Itertools;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc0xx::abc095::a_something_on_it::run(String::from("oxo")));
}
