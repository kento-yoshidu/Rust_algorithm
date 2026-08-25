// https://atcoder.jp/contests/awc0033/tasks/awc0033_a

fn run(_n: usize, k: usize, s: &str, t: &str) -> usize {
    let count = s.chars()
        .zip(t.chars())
        .filter(|(s, t)| s != t)
        .count();

    if count >= k {
        count - k
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str, usize);

    #[test]
    fn awc0033_a() {
        let tests = [
            TestCase(5, 2, "10100", "01010", 2),
        ];

        for TestCase(n, k, s, t, expected) in tests {
            assert_eq!(run(n, k, s, t), expected);
        }
    }
}
