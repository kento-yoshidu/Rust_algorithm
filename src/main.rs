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
    atcoder::abc2xx::abc293::a_swap_odd_and_even::run(String::from("abcdef"));
}

// Todo
// abc147 b
// abc294 a

// Refactoring
// abc197 b
