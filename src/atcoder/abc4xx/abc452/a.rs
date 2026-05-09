// https://atcoder.jp/contests/abc452/tasks/abc452_a

fn run(m: usize, d: usize) -> &'static str {
    if m == 1 && d == 7 ||
       m == 3 && d == 3 ||
       m == 5 && d == 5 ||
       m == 7 && d == 7 ||
       m == 9 && d == 9 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc452_a() {
        let tests = [
            TestCase(3, 3, "Yes"),
            TestCase(1, 1, "No"),
            TestCase(4, 4, "No"),
            TestCase(11, 7, "No")
        ];

        for TestCase(m, d, expected) in tests {
            assert_eq!(run(m, d), expected);
        }
    }
}
