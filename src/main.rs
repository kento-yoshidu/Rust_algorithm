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
mod aoj;

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
    println!("{}", atcoder::abc0xx::abc087::b_coins::run(2, 2, 2, 100));
    //=> 2
    println!("{}", atcoder::abc0xx::abc087::b_coins::run(5, 1, 0, 150));
    //=> 0
    println!("{}", atcoder::abc0xx::abc087::b_coins::run(30, 40, 50, 6000));
    //=> 213
}

// Todo
// abc147 b
// abc294 a

// Refacotring
// abc197 b
