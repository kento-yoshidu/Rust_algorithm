// https://atcoder.jp/contests/abc379/tasks/abc379_b

fn run(n: usize, k: usize, s: &str) -> usize {
    let mut vec: Vec<char> = s.chars().collect();

    let mut ans = 0;

    for i in 0..=n-k {
        if (i..i+k).all(|k| {
            vec[k] == 'O'
        }) {
            ans += 1;

            for k in i..i+k {
                vec[k] = 'X';
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 3, "OOXOOOO", 1),
            TestCase(12, 2, "OXXOOOXOOOOX", 3),
            TestCase(22, 5, "XXOOOOOOOOXXOOOOOXXXXX", 2),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
