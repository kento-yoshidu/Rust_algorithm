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
    println!("{}", atcoder::abc2xx::abc205::b_permutation_check::run(5, vec![3, 1, 2, 4, 5]));
    println!("{}", atcoder::abc2xx::abc205::b_permutation_check::run(6, vec![3, 1, 4, 1, 5, 2]));
}

// Todo
// abc294 a
