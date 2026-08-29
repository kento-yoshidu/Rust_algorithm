// https://atcoder.jp/contests/abc056/tasks/abc056_b

fn run(w: isize, a: isize, b: isize) -> isize {
    if (a - b).abs() <= w {
        0
    } else {
        (a - b).abs() - w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn abc056_b() {
        let tests = [
            TestCase(3, 2, 6, 1),
            TestCase(3, 1, 3, 0),
            TestCase(5, 10, 1, 4),
        ];

        for TestCase(w, a, b, expected) in tests {
            assert_eq!(run(w, a, b), expected);
        }
    }
}
