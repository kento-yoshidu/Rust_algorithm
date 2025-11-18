// https://atcoder.jp/contests/abc215/tasks/abc215_c

use itertools::Itertools;

fn run(s: &str, k: usize) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut vec: Vec<_> = Vec::new();

    for a in chars.iter().permutations(s.len()) {
        vec.push(a.iter().map(|c| c.to_string()).collect::<String>());
    }

    vec.sort();
    vec.dedup();

    vec[k-1].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, &'static str);

    #[test]
    fn abc215_c() {
        let tests = [
            TestCase("aab", 2, "aba"),
            TestCase("baba", 4, "baab"),
            TestCase("ydxwacbz", 40320, "zyxwdcba"),
        ];

        for TestCase(s, k, expected) in tests {
            assert_eq!(run(s, k), expected);
        }
    }
}
