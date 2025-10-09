// https://atcoder.jp/contests/abc160/tasks/abc160_b

fn run(x: usize) -> usize {
    let a = x / 500;
    let b = (x - a * 500) / 5;

    a * 1000 + b * 5
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc160_b() {
        let tests = [
            TestCase(1024, 2020),
            TestCase(0, 0),
            TestCase(1000000000, 2000000000),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
