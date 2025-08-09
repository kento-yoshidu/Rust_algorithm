// https://atcoder.jp/contests/abc123/tasks/abc123_b

fn run(a: usize, b: usize, c: usize, d: usize, e: usize) -> usize {
    let vec = vec![a, b, c, d, e];

    let mut ans = std::usize::MAX;

    for i in 0..5 {
        let mut total = 0;

        for (index, a) in vec.iter().enumerate() {
            if i != index {
                if a % 10 == 0 {
                    total += a;
                } else {
                    total += a + (10 - a % 10)
                }
            } else {
                total += a
            }
        }

        ans = ans.min(total);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize);

    #[test]
    fn abc123_b() {
        let tests = [
            TestCase(29, 20, 7, 35, 120, 215),
            TestCase(101, 86, 119, 108, 57, 481),
            TestCase(123, 123, 123, 123, 123, 643),
        ];

        for TestCase(a, b, c, d, e, expected) in tests {
            assert_eq!(run(a, b, c, d, e), expected);
        }
    }
}
