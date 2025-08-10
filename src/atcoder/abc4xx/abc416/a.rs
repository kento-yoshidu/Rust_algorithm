// https://atcoder.jp/contests/abc416/tasks/abc416_a

fn run(_n: usize, l: usize, r: usize, s: &str) -> &'static str {
    for i in l-1..r {
        if s.chars().nth(i).unwrap() == 'x' {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str, &'static str);

    #[test]
    fn abc416_a() {
        let tests = [
            TestCase(10, 6, 8, "xoxxooooxo", "Yes"),
            TestCase(9, 6, 8, "xoxxoxoox", "No"),
        ];

        for TestCase(n, l, r, s, expected) in tests {
            assert_eq!(run(n, l, r, s), expected);
        }
    }
}
