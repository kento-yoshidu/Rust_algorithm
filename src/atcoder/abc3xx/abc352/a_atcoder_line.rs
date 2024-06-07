// https://atcoder.jp/contests/abc349/tasks/abc352_a

fn run(n: usize, x: usize, y: usize, z: usize) -> &'static str {
    if (x < z && z < y) || (y < z && z < x) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 6, 1, 3, "Yes"),
            TestCase(10, 3, 2, 9, "No"),
            TestCase(100, 23, 67, 45, "Yes"),
        ];

        for TestCase(n, x, y, z, expected) in tests {
            assert_eq!(run(n, x, y, z), expected);
        }
    }
}
