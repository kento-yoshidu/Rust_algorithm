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
    println!("{}", atcoder::abc1xx::abc104::b_accepted::run(String::from("AtCoder")));
    println!("{}", atcoder::abc1xx::abc104::b_accepted::run(String::from("AtCoder")));
    println!("------------------------");
    println!("{}", atcoder::abc1xx::abc104::b_accepted::run(String::from("BtCoder")));
    // println!("{}", atcoder::abc1xx::abc104::b_accepted::run(String::from("AcycliC")));
}

// Todo
// abc294 a
