// https://atcoder.jp/contests/abc093/tasks/arc094_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
    let mut vec = vec![a, b, c];
    vec.sort();

    let a = vec[0];
    let b = vec[1];
    let c = vec[2];

    if (c-a) % 2 == 0 && (c-b) % 2 == 0 {
        (c-a) / 2 + (c-b) / 2
    } else if ((c-a) % 2 == 0 && (c-b) % 2 == 1) || ((c-a) % 2 == 1 && (c-b) % 2 == 0) {
        (c-a) / 2 + (c-b) / 2 + 2
    } else {
        (c-a) / 2 + (c-b) / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 5, 4, 2),
            TestCase(2, 5, 4, 2),
            TestCase(2, 6, 3, 5),
            TestCase(31, 41, 5, 23),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(expected, run(a, b, c));
        }
    }
}
