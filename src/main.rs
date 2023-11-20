#![allow(dead_code)]
// mod aoj;
mod atcoder;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
mod yukicoder;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc1xx::abc167::b_easy_linear_programming::run(2, 1, 1, 3));
    println!("{}", atcoder::abc1xx::abc167::b_easy_linear_programming::run(1 ,2 ,3 ,4));
    println!("{}", atcoder::abc1xx::abc167::b_easy_linear_programming::run(2000000000, 0, 0, 2000000000));
    println!("{}", atcoder::abc1xx::abc167::b_easy_linear_programming::run(0, 0, 2000000000, 2000000000));
}
