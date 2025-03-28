// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_b

fn run(x: usize, y: usize, z: usize) -> usize {
    if x + y <= z {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 4, 0),
            TestCase(3, 4, 10, 1),
        ];

        for TestCase(x, y, z, expected) in tests {
            assert_eq!(run(x, y, z), expected);
        }
    }
}
