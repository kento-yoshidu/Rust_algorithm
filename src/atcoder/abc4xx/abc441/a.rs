// https://atcoder.jp/contests/abc441/tasks/abc441_a

fn run(p: usize, q: usize, x: usize, y: usize) -> &'static str {
    if x < p || y < q || p+99 < x || q+99 < y {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn abc441_a() {
        let tests = [
            TestCase(3, 3, 5, 10, "Yes"),
            TestCase(5, 5, 10, 1000, "No"),
            TestCase(1, 2, 1, 1, "No"),
        ];

        for TestCase(p, q, x, y, expected) in tests {
            assert_eq!(run(p, q, x, y), expected);
        }
    }
}
