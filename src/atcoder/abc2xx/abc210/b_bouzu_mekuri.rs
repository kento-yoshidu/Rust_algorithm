// https://atcoder.jp/contests/abc210/tasks/abc210_b

fn run(_n: usize, s: &str) -> &'static str {
    s.chars()
        .enumerate()
        .find(|(_, c)| {
            *c == '1'
        })
        .map(|(i, _)| {
            if i % 2 == 0 {
                "Takahashi"
            } else {
                "Aoki"
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc210_b() {
        let tests = [
            TestCase(5, "00101", "Takahashi"),
            TestCase(3, "010","Aoki"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
