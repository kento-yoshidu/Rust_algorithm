# Rust_algorithm

## memo

[Why chars().nth(index).unwrap() is so slow?](https://www.reddit.com/r/rust/comments/tbsffu/why_charsnthindexunwrap_is_so_slow/?rdt=43355)

## AtCoder Template

```rust
#[allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::collections::{BtreeMap, BtreeSet, HashMap, HashSet};
use std::cmp::{min, max};

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

## Todo

- abc061 b
- abc147 b
- abc183 d
- abc187 c (TLE)
- abc221 c
- abc247 d
- abc253 b
- abc259 c
- abc272 b
- abc294 a
- abc296 a run2
- abc326 c
- abc275 b
- abc303 c
- abc328 c

## Refactoring

- abc197 b
- abc237 c
- tessoku bc
- joigsp2023 a
