// https://atcoder.jp/contests/abc465/tasks/abc465_b

fn run(x: usize, y: usize, l: usize, r: usize, a: usize, b: usize) -> usize {
    (a..b)
        .map(|i| {
            if (l..r).contains(&i) {
                x
            } else {
                y
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize, usize);

    #[test]
    fn abc465_b() {
        let tests = [
            TestCase(700, 300, 9, 17, 7, 21, 7400),
            TestCase(600, 500, 9, 17, 17, 20, 1500),
            TestCase(900, 200, 12, 14, 11, 13, 1100),
        ];

        for TestCase(x, y, l, r, a, b, expected) in tests {
            assert_eq!(run(x, y, l, r, a, b), expected);
        }
    }
}
