// https://atcoder.jp/contests/indeednow-qualb/tasks/indeednow_2015_qualb_1

fn run(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1-x2).abs() + (y1-y2).abs() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, 2, 5, 4),
            TestCase(1, 1, 1, 2, 2),
            TestCase(20, 40, 32, 64, 37),
        ];

        for TestCase(x1, y1, x2, y2, expected) in tests {
            assert_eq!(run(x1, y1, x2, y2), expected);
        }
    }
}
