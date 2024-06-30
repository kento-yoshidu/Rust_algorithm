// https://atcoder.jp/contests/arc053/tasks/arc053_a

fn run(h: usize, w: usize) -> usize {
    (h-1)*w + (w-1)*h
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 7),
            TestCase(4, 1, 3),
            TestCase(1, 1, 0),
        ];

        for TestCase(h, w, expected) in tests {
            assert_eq!(run(h, w), expected);
        }
    }
}
