#![allow(dead_code)]
mod aoj;
mod atcoder;
mod basic;
// mod nlp_100_knock;
// mod project_euler;
mod yukicoder;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::other_contest::kupc2017::a_credits::run(5, 15, vec![3, 8, 2, 5, 6]));
    println!("{}", atcoder::other_contest::kupc2017::a_credits::run(5, 25, vec![3, 8, 2, 5, 6]));
    println!("{}", atcoder::other_contest::kupc2017::a_credits::run(4, 20, vec![1, 1, 20, 19]));

}
