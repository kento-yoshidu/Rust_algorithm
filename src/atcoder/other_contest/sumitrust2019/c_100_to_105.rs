// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c

pub fn run(x: usize) -> usize {
    for i in 1..=x/100 {
        if 100*i <= x && x <= 105*i {
            return 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(615, 1),
            TestCase(217, 0),
            TestCase(1, 0),
            TestCase(100, 1),
            TestCase(103, 1),
            TestCase(105, 1),
            TestCase(106, 0),
            TestCase(152, 0),
            TestCase(200, 1),
            TestCase(210, 1),
            TestCase(211, 0),
            TestCase(1868, 1),
            TestCase(1995, 1),
            TestCase(1996, 0),
            TestCase(2019, 1),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
