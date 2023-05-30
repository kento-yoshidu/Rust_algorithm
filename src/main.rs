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

fn main() {
    println!("Hello, world!");

    println!("{}", atcoder::abc143::b_takoyaki_festival_2019::run(3, vec![3, 1, 2]));
}
