// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_h

fn run(n: usize, s: usize) -> usize {
    let mut ans = 0;

    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase(usize, usize, usize);

    #[test]
    fn ma008() {
        let tests = [
            TestCase(3, 4, 6),
            TestCase(869, 120, 7140)
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
