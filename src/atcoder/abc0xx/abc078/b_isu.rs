// https://atcoder.jp/contests/abc078/tasks/abc078_b

fn run(x: i32, y: i32, z: i32) -> i32 {
    (x - z) / (y + z)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, i32, i32);

    #[test]
    fn test () {
        let tests = [
            TestCase(13, 3, 1, 3),
            TestCase(12, 3, 1, 2),
            TestCase(100000, 1, 1, 49999),
            TestCase(64146, 123, 456, 110),
            TestCase(64145, 123, 456, 109),
        ];

        for TestCase(x, y, z, expected) in tests {
            assert_eq!(run(x, y, z), expected);
        }
    }
}
