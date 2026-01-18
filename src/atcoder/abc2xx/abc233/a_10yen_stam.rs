// https://atcoder.jp/contests/abc233/tasks/abc233_a

fn run(x: usize, y: usize) -> usize {
    if y < x {
        return 0;
    }

    ((y - x) as f64 / 10.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc233_a() {
        let tests = [
            TestCase(80, 94, 2),
            TestCase(1000, 63, 0),
            TestCase(270, 750, 48),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
