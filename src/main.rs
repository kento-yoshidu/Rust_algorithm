#![allow(dead_code)]
// mod aoj;
mod atcoder;
// mod nlp_100_knock;
// mod project_euler;

fn main() {
    println!("Hello World");
    println!("{}", atcoder::abc0xx::abc003::b_playing_cards::run(String::from("ch@ku@ai"), String::from("choku@@i")));
    println!("{}", atcoder::abc0xx::abc003::b_playing_cards::run(String::from("aoki"), String::from("@ok@")));
    println!("{}", atcoder::abc0xx::abc003::b_playing_cards::run(String::from("arc"), String::from("abc")));
}
