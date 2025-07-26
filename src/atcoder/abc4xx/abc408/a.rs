// https://atcoder.jp/contests/abc408/tasks/abc408_a

fn run(_n: usize, s: isize, t: Vec<isize>) -> &'static str {
    if t[0] > s {
        return "No";
    }

    if t.windows(2)
        .any(|w| (w[1] - w[0]).abs() > s) {
            "No"
        } else {
            "Yes"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, &'static str);

    #[test]
    fn abc408_a() {
        let tests = [
            TestCase(5, 10, vec![6, 11, 21, 22, 30], "Yes"),
            TestCase(2, 100, vec![1, 200], "No"),
            TestCase(10, 22, vec![47, 81, 82, 95, 117, 146, 165, 209, 212, 215], "No"),
        ];

        for TestCase(n, s, t, expected) in tests {
            assert_eq!(run(n, s, t), expected);
        }
    }
}
