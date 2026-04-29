// https://atcoder.jp/contests/abc296/tasks/abc296_a

fn run(n: usize, s: &str) -> &'static str {
    for i in 0..n - 1 {
        if s.chars().nth(i).unwrap() == s.chars().nth(i + 1).unwrap() {
            return "No";
        }
    }

    "Yes"
}

fn run2(_n: usize, s: &str) -> &'static str {
    if s.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .all(|v| { v[0] != v[1] }
    ) {
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
    fn abc296_a() {
        let tests = [
            TestCase(6, "MFMFMF", "Yes"),
            TestCase(9, "FMFMMFMFM", "No"),
            TestCase(1, "F", "Yes"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
