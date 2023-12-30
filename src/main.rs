#![allow(dead_code)]
// mod aoj;
mod atcoder;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
mod yukicoder;

fn main() {
    println!("Hello World");
    println!("{:?}", atcoder::abc1xx::abc173::b_judge_status_summary::run2(6, vec!["AC", "TLE", "AC", "AC", "WA", "TLE"]));
}
