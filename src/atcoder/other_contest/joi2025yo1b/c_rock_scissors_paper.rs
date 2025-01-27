// https://atcoder.jp/contests/joi2025yo1b/tasks/joi2025_yo1b_c

fn run(_n: usize, s: &str, t: &str) -> (usize, usize) {
    s.chars()
        .zip(t.chars())
        .fold((0, 0), |acc, (s, t)| {
            if s == t {
                acc
            } else if s == 'R' {
                (acc.0, acc.1+1)
            } else {
                if t == 'R' {
                    (acc.0, acc.1+1)
                } else {
                    (acc.0+1, acc.1)
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(3, "RSR", "PPR", (1, 1)),
            TestCase(5, "RRRRR", "PPPPP", (0, 5)),
            TestCase(4, "RSRR", "RPRR", (1, 0)),
            TestCase(6, "RSSRSS", "PPRRRP", (2, 3)),
        ];

        for TestCase(n, s, t, expected) in tests {
            assert_eq!(run(n, s, t), expected);
        }
    }
}
