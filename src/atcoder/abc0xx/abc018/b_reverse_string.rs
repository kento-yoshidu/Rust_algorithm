// https://atcoder.jp/contests/abc018/tasks/abc018_2

fn run(s: &str, _n: usize, l: Vec<(usize, usize)>) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    for (l, r) in l {
        chars[l-1..=r-1].reverse();
    }

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("abcdef", 2, vec![(3, 5), (1, 4)], "debacf"),
            TestCase("redcoat", 3, vec![(1, 7), (1, 2), (3, 4)], "atcoder"),
        ];

        for TestCase(s, n, l, expected) in tests {
            assert_eq!(run(s, n, l), expected);
        }
    }
}
