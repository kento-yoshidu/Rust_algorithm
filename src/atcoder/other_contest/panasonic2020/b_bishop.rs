// https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_b

pub fn run(h: usize, w: usize) -> usize {
    if h == 1 || w == 1 {
        return 1;
    }

    if h*w % 2 == 0 {
        h * w / 2
    } else {
        (h * w + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 5, 10),
            TestCase(7, 3, 11),
            TestCase(1000000000, 1000000000, 500000000000000000),
        ];

        for TestCase(h, w, expected) in tests {
            assert_eq!(expected, run(h, w));
        }
    }
}
