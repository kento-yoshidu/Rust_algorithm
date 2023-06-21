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

    println!("{}", aoj::atp1::atp1_1_d_watch::run(46979));
}

// Todo
// abc147 b
// abc294 a
