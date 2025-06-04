# Rust_algorithm

## memo

[Why chars().nth(index).unwrap() is so slow?](https://www.reddit.com/r/rust/comments/tbsffu/why_charsnthindexunwrap_is_so_slow/?rdt=43355)

## AtCoder Template

```rust
#[allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::collections::{BtreeMap, BtreeSet, HashMap, HashSet, VecDeque};
use std::cmp::{min, max};

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

## 未提出

- abc035 b
- abc183 d
- abc247 d
- abc326 c
