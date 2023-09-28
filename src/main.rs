#![allow(dead_code)]
mod aoj;
mod atcoder;
mod nlp_100_knock;
mod project_euler;

fn main() {
    println!("Hello, world!");
    println!("{:?}", atcoder::abc2xx::abc297::c_pc_on_the_table::run(3, 5, vec![vec!['T', 'T', 'T', '.', '.'], vec!['.', 'T', 'T', 'T', '.'], vec!['T', 'T', 'T', 'T', 'T']]));
}
