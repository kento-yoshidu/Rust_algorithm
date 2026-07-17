// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/2/ITP1_2_B

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a < b && b < c {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn itp1_2_b() {
        let tests = [
            TestCase(1, 3, 8, "Yes"),
            TestCase(3, 8, 1, "No"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
