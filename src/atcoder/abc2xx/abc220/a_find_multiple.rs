// https://atcoder.jp/contests/abc220/tasks/abc220_a

fn run(a: isize, b: isize, c: isize) -> isize {
    (a..=b)
        .find(|num| {
            num % c == 0
        })
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn abc220_a() {
        let tests = [
            TestCase(123, 456, 100, 200),
            TestCase(630, 940, 314, -1),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
