// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run<'a>(_n: usize, k: usize, s: Vec<&'a str>) -> Vec<&'a str> {
    let mut vec = s.clone();

    vec = vec[0..k].to_vec();

    vec.sort();

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc288_b() {
        let tests = [
            TestCase(5, 3, vec!["abc", "aaaaa", "xyz", "a", "def"], vec!["aaaaa", "abc", "xyz"]),
            TestCase(4, 4, vec!["z", "zyx", "zzz", "rbg"], vec!["rbg", "z", "zyx", "zzz"]),
            TestCase(3, 1, vec!["abc", "arc", "agc"], vec!["abc"]),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
