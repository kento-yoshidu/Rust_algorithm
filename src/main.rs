#![allow(dead_code)]
// mod aoj;
mod atcoder;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
mod yukicoder;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc0xx::abc032::c_row::run(7, 6, vec![4, 3, 1, 1, 2, 10, 2]));
    println!("{}", atcoder::abc0xx::abc032::c_row::run(6, 10, vec![10, 10, 10, 10, 0, 10]));
    println!("{}", atcoder::abc0xx::abc032::c_row::run(6, 9, vec![10, 10, 10, 10, 10 , 10]));
}
