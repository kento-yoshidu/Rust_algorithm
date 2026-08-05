// https://atcoder.jp/contests/abc020/tasks/abc020_b

fn run(a: usize, b: usize) -> usize {
    let num = a.to_string() + &b.to_string();

    num.parse::<usize>().unwrap() * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc020_b() {
        let tests = [
            TestCase(1, 23, 246),
            TestCase(999, 999, 1999998),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
