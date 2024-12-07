// https://atcoder.jp/contests/abc047/tasks/abc047_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    let mut vec = vec![a, b, c];

    vec.sort();

    if vec[0] + vec[1] == vec[2] {
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
    fn test() {
        let tests = [
            TestCase(10, 30, 20, "Yes"),
            TestCase(30, 30, 100, "No"),
            TestCase(56, 25, 31, "Yes"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
