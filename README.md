# Rust_algorithm

## AtCoder Template

```rust
#[allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::collections::{BtreeMap, BtreeSet, HashMap, HashSet, VecDeque};
use std::cmp::{min, max, Ordering};

fn lower_bound<T: Ord>(vec: &[T], value: T) -> usize {
    vec.binary_search_by(|x| {
        if *x < value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
    .err()
    .unwrap()
}

fn upper_bound<T: Ord>(vec: &[T], value: T) -> usize {
    vec.binary_search_by(|x| {
        if *x <= value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
    .err()
    .unwrap()
}

// n以下の最大の数を返す
fn max_under_n<T: Ord>(vec: &[T], value: T) -> Option<usize> {
    vec.binary_search_by(|x| {
        if *x <= value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
    .err()
    .map(|x| if x == 0 {
        None
    } else {
        Some(x - 1)
    })
    .flatten()
}

#[allow(unused)]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    for i in a {
        ans += i;
    }

    println!("{ans}");
}
```

## memo

[Why chars().nth(index).unwrap() is so slow?](https://www.reddit.com/r/rust/comments/tbsffu/why_charsnthindexunwrap_is_so_slow/?rdt=43355)
