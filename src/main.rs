#![allow(dead_code)]
mod aoj;
mod atcoder;
mod nlp_100_knock;
mod project_euler;

#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc2xx::abc223::a_exact_price::run(0));
}
