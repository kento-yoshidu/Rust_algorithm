// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_c

use itertools::Itertools;

fn run(n: usize, s: &str) -> &'static str {
    let len = s.chars()
        .sorted()
        .dedup()
        .collect::<Vec<char>>()
        .len();

    if len >= 3 {
        "Yes"
    } else {
        "No"
    }
}
