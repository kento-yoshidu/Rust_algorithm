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
    // atcoder::tessoku_book::axx::a06_how_many_guests::run(5, 1, vec![11, 46, 47, 77, 80], vec![vec![2, 3]])}
    println!("{:?}", atcoder::abc0xx::abc086::c_otoshidama::run(9, 46000));
}

// Todo
// abc147 b
// abc294 a

// Refacotring
// abc197 b
