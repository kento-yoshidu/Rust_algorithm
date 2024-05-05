// https://atcoder.jp/contests/abc349/tasks/abc352_b

pub fn run(s: &str, t: &str) -> Vec<usize> {
    let mut ans = vec![false; t.len()];

    for (i, c) in t.chars().enumerate() {
        if !s.contains(c) {
            ans[i] = true;
        }
    }

    ans.into_iter()
        .enumerate()
        .filter(|(_, b)| *b == false)
        .map(|(i, _)| i + 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase("abc", "axbxyc", vec![1, 3, 6]),
            TestCase("aaaa", "bbbbaaaa", vec![5, 6, 7, 8]),
            TestCase("atcoder", "atcoder", vec![1, 2, 3, 4, 5, 6, 7]),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
