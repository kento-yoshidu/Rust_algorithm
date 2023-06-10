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
    println!("{}", atcoder::abc1xx::abc113::b_palace::run(2, 12, 5, vec![1000, 2000]));
    println!("{}", atcoder::abc1xx::abc113::b_palace::run(3, 21, -11, vec![81234, 94124, 52141]));
}

// Todo
// abc294 a
