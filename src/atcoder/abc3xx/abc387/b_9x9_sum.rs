// https://atcoder.jp/contests/abc387/tasks/abc387_b

pub fn run(x: usize) -> usize {
    let mut ans = 0;

    for i in 1..=9 {
        for j in 1..=9 {
            if i*j != x {
                ans += i*j;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2024),
            TestCase(11, 2025),
            TestCase(24, 1929),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
