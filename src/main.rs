#![allow(dead_code)]
// mod aoj;
mod atcoder;
// mod paiza;
// mod dp;
// mod basic;
// mod nlp_100_knock;
// mod project_euler;
// mod yukicoder;
// mod other;

fn run(s: &str) -> usize {
    let n = s.len();
    let s: Vec<char> = s.chars().collect();
    let mut ans = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n && (r <= l || s[r] != s[r-1]) {
            r += 1;
        }

        ans = (ans + (r - l)) % 998244353;

        if l == r {
            r += 1;
        }
    }

    ans
}

fn main() {
    let ans = run("abcd");
    // let ans = run("cabcabcbcaccacbcbcaabacbacaabccacbccbcacbacbacabcacabcaccaaaaabababcbabacaccabbcacbcbcbcababcbcbabca");
    println!("{}", ans);
}
