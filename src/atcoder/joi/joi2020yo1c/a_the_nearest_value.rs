// https://atcoder.jp/contests/joi2020yo1c/tasks/joi2020_yo1c_a

fn run(x: usize, l: usize, r: usize) -> usize {
    if l <= x && x <= r {
        x
    } else if x < l {
        l
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 3, 6, 6),
            TestCase(7, 3, 10, 7),
            TestCase(8, 10, 10, 10),
        ];

        for TestCase(x, l, r, expected) in tests {
            assert_eq!(run(x, l, r), expected);
        }
    }
}
