#![allow(dead_code)]
// mod aoj;
mod atcoder;
mod basic;
// mod nlp_100_knock;
// mod project_euler;
mod yukicoder;

fn main() {
    println!("Hello World");
    atcoder::abc2xx::abc294::b_ascii_art::run(3, 3, vec![vec![24, 0, 0], vec![0, 25, 0], vec![0, 0, 26]]);
}
