// https://atcoder.jp/contests/abc434/tasks/abc434_a

fn run(w: usize, b: usize) -> usize {
    (w * 1000) / b + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc434_a() {
        let tests = [
            TestCase(80, 5, 16001),
            TestCase(70, 6, 11667),
            TestCase(100, 100, 1001),
        ];

        for TestCase(w, b, expected) in tests {
            assert_eq!(run(w, b), expected);
        }
    }
}
