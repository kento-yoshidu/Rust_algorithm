// https://atcoder.jp/contests/jsc2021/tasks/jsc2021_a

fn run(x: usize, y: usize, z: usize) -> usize {
    let tmp = y as f64 / x as f64;

    (z as f64 * tmp - 1.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(100, 200, 100, 199),
            TestCase(103, 971, 593, 5590),
            TestCase(1000, 1, 1, 0),
        ];

        for TestCase(x, y, z, expected) in tests {
            assert_eq!(run(x, y, z), expected);
        }
    }
}
