// https://atcoder.jp/contests/abc144/tasks/abc144_b

fn run (n: usize) -> &'static str {
    if n > 81 {
        return "No";
    }

    for i in 0..=9 {
        for j in 0..=9 {
            if i * j == n {
                return "Yes";
            }
        }
    }

    "No"
}

fn run2(n: usize) -> &'static str {
    use itertools::Itertools;

    if (1..=9).combinations_with_replacement(2)
        .any(|t| {
            t[0] * t[1] == n
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc144_b() {
        let tests = [
            TestCase(10, "Yes"),
            TestCase(50, "No"),
            TestCase(81, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
