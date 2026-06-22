# Rust_algorithm

## AtCoder Template

## 改善が必要

- ABC001 D - 感雨時刻の整理

## memo

[Why chars().nth(index).unwrap() is so slow?](https://www.reddit.com/r/rust/comments/tbsffu/why_charsnthindexunwrap_is_so_slow/?rdt=43355)

## Not AC

### BFS

- [ABC226 C - Martial artist](https://atcoder.jp/contests/abc226/tasks/abc226_c)

### ナップサック

- [ABC054 D - Mixing Experiment](https://atcoder.jp/contests/abc054/tasks/abc054_d)

### ダイクストラ

- [ABC252 E - Road Reduction](https://atcoder.jp/contests/abc252/tasks/abc252_e)
- [ABC355 E - Non-Decreasing Colorful Path](https://atcoder.jp/contests/abc335/tasks/abc335_e)(TLE)

### ワーシャルフロイド

- https://atcoder.jp/contests/abc143/tasks/abc143_e

### 01-BFS

- [E - Stronger Takahashi](https://atcoder.jp/contests/abc213/tasks/abc213_e)

### Union Find

- [ABC049 D - 連結](https://atcoder.jp/contests/abc049/tasks/arc065_b)
- [ABC270 F - Transportation](https://atcoder.jp/contests/abc270/tasks/abc270_f)
- [ABC352 E - Clique Connect](https://atcoder.jp/contests/abc352/tasks/abc352_e)
- [ARC106 B - Values](https://atcoder.jp/contests/arc106/tasks/arc106_b)

### 重み付きUF

- [ABC280 F - Pay or Receive](https://atcoder.jp/contests/abc280/tasks/abc280_f)

```rust
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}
```