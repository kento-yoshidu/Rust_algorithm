// https://atcoder.jp/contests/arc109/tasks/arc109_a

use std::cmp::min;

pub fn run(a: isize, b: isize, x: isize, y: isize) -> isize {
    if a < b {
        x + (b - a) * min(2 * x, y)
    } else if a > b {
        x + (a - b - 1) * min(2 * x, y)
    } else {
        x
    }
}