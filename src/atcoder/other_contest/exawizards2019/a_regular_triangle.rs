// https://atcoder.jp/contests/exawizards2019/tasks/exawizards2019_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a == b && b == c {
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
            TestCase(2, 2, 2, "Yes"),
            TestCase(3, 4, 5, "No")
        ];

        for TestCase(a, b, c ,expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
