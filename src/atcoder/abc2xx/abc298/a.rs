// https://atcoder.jp/contests/abc298/tasks/abc298_a

fn run(_n: usize, s: &str) -> &'static str {
    let ok = s.contains('o');
    let reject = s.contains('x');

    if ok && !reject {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc298_a() {
        let tests = [
            TestCase(4, "oo--", "Yes"),
            TestCase(3, "---", "No"),
            TestCase(1, "o", "Yes"),
            TestCase(100, "ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooox", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
