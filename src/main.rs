#![allow(dead_code)]
// mod aoj;
mod atcoder;
mod basic;
// mod nlp_100_knock;
// mod project_euler;
mod yukicoder;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc2xx::abc212::b_weak_password::run("1239"));
    println!("{}", atcoder::abc2xx::abc212::b_weak_password::run("1234"));
    println!("{}", atcoder::abc2xx::abc212::b_weak_password::run("9012"));
}
