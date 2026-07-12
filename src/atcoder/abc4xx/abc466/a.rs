// https://atcoder.jp/contests/abc466/tasks/abc466_a

fn run(_n: usize, x: Vec<isize>) -> &'static str {
    if x.into_iter().all(|x| x < 0) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, &'static str);

    #[test]
    fn abc466_a() {
        let tests = [
            TestCase(4, vec![2, 0, -1, 2], "No"),
            TestCase(3, vec![-5, -2, -1], "Yes"),
            TestCase(4, vec![0, -2, 0, -1], "No"),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
