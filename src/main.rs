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
    println!("{}", atcoder::tessoku_book::axx::a05_three_cards::run(3000, 4000));
}

// Todo
// abc147 b
// abc294 a

// Refacotring
// abc197 b
