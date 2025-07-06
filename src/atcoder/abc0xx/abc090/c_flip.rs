// https://atcoder.jp/contests/abc090/tasks/arc091_a

use std::cmp::{min, max};

pub fn run(n: usize, m: usize) -> usize {
    let n = min(n, m);
    let m = max(n, m);

    if n == 1 {
        if m == 1 {
            1
        } else {
            m - 2
        }
    } else {
        (n - 2) * (m - 2)
    }
}
