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
    println!("{}", atcoder::abc1xx::abc158::c_tax_increase::run(2.0, 2.0));
    println!("{}", atcoder::abc1xx::abc158::c_tax_increase::run(8.0, 10.0));
    println!("{}", atcoder::abc1xx::abc158::c_tax_increase::run(19.0, 99.0));
}

// Todo
// abc294 a
