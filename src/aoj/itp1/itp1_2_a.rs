// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/2/ITP1_2_A

fn run(a: isize, b: isize) -> &'static str {
    if a > b {
        "a > b"
    } else if a < b {
        "a < b"
    } else {
        "a == b"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, &'static str);

    #[test]
    fn itp1_2_a() {
        let tests = [
            TestCase(1, 2, "a < b"),
            TestCase(4, 3, "a > b"),
            TestCase(5, 5, "a == b"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
