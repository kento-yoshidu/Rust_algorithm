// https://atcoder.jp/contests/abc044/tasks/abc044_b

use itertools::Itertools;

pub fn run(w: String) -> String {
    let hashmap = w.chars().counts();

    if hashmap.iter().all(|(_, value)| {
        value % 2 == 0
    }) {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

fn main() {
    println!("{}", run(String::from("abaccaba")));
    println!("{}", run(String::from("hthth")));
}
