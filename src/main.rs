#![allow(dead_code)]
mod atcoder;
mod nlp_100_knock;
mod aoj;

fn main() {
    println!("Hello, world!");
    println!("{}", atcoder::tessoku_book::axx::a12_printer::run(4, 10, vec![1, 2, 3, 4]));
}
