// https://atcoder.jp/contests/abc311/tasks/abc311_a

fn run(_n: usize, s: &str) -> usize {
    let mut ans = 0;

    for abc in ['A', 'B', 'C'] {
        ans = ans.max(s.chars().position(|c| c == abc).unwrap())
    }

    ans + 1
}

fn run2(_n: usize, s: &str) -> usize {
    ['A', 'B', 'C']
        .into_iter()
        .map(|abc| {
            s.chars()
                .position(|c| c == abc)
                .unwrap()
        })
        .max()
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn abc311_a() {
        let tests = [
            TestCase(5, "ACABB", 4),
            TestCase(4, "CABC", 3),
            TestCase(30, "AABABBBABABBABABCABACAABCBACCA", 17),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
