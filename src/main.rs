/* AtCoder Template
use proconio::input;

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
    println!("{}", atcoder::abc0xx::abc090::b_palindromic_numbers::run(11009, 11332));
    println!("{}", atcoder::abc0xx::abc090::b_palindromic_numbers::run(31415, 92653));
}

// Todo
// abc294 a
