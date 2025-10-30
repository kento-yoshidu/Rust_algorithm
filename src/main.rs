#![allow(dead_code)]
// mod aoj;
mod atcoder;
mod paiza;
// mod dp;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
// mod yukicoder;

use itertools::Itertools;
use std::cmp::min;

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || h as isize <= i || w as isize <= j
}

fn run(t: usize, tc: Vec<(usize, usize, usize)>) {
    for (a, b, c) in tc {
        // a < b < c
        if a < b && b < c {
            println!("{a}");
        } else if a == b && b == c {
        // a = b = c
            println!("{a}");
        } else if b < a && a < c {
        // b < a < c
            let mut count = 0;
            count += b;

            let temp = min(a, c);
            count += temp - b;

            println!("{count}");
        } else if c < a && a < b {
            println!("{c}");
        } else {
            println!("{b}");
        }
    }
}

fn main() {
    run(5, vec![(3, 1, 2), (100, 0, 0), (1000000, 1000000, 1000000), (31, 41, 59), (1000000000, 10000, 1)]);
}
