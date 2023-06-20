/* AtCoder Template
// use proconio::input;

fn main() {
    input! {
        a: i32,
        p: i32
    }
    let all_p = a * 3 +p;
    println!("{}", all_p / 2);
}
*/

mod atcoder;
mod nlp_100_knock;

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
    println!("{}", atcoder::abc0xx::abc064::b_break_number::run(7));
    println!("{}", atcoder::abc0xx::abc064::b_break_number::run(32));
    println!("{}", atcoder::abc0xx::abc064::b_break_number::run(1));
    println!("{}", atcoder::abc0xx::abc064::b_break_number::run(100));
}

// Todo
// abc294 a
