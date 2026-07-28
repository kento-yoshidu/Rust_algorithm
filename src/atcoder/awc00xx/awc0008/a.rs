// https://atcoder.jp/contests/awc0008/tasks/awc0008_a

fn run(n: usize, w: usize, k: usize) -> &'static str {
    if k <= w / (n - 1) {
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
    fn awc0008_a() {
        let tests = [
            TestCase(5, 100, 20, "Yes"),
            TestCase(10, 180, 25, "No"),
            TestCase(51, 500, 15, "No"),
        ];

        for TestCase(n, w, k, expected) in tests {
            assert_eq!(run(n, w, k), expected);
        }
    }
}
