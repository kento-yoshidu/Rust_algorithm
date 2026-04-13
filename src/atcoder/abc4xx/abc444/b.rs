// https://atcoder.jp/contests/abc444/tasks/abc444_b

fn sum(num: String) -> u32 {
    num.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

fn run(n: usize, k: usize) -> usize {
    (1..=n)
        .filter(|n| {
            sum(n.to_string()) == k as u32
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc444_b() {
        let tests = [
            TestCase(30, 4, 3),
            TestCase(2026, 10, 121),
            TestCase(99999, 45, 1),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
