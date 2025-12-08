// https://atcoder.jp/contests/abc184/tasks/abc184_b

fn run(_n: usize, x: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.into_iter()
        .fold(x, |state, char| {
            if char == 'o' {
                state + 1
            } else {
                if state == 0 {
                    0
                } else {
                    state - 1
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, usize);

    #[test]
    fn abc184_b() {
        let tests = [
            TestCase(3, 0, "xox", 0),
            TestCase(20, 199999, "oooooooooxoooooooooo", 200017),
            TestCase(20, 10, "xxxxxxxxxxxxxxxxxxxx", 0),
        ];

        for TestCase(n, x, s, expected) in tests {
            assert_eq!(run(n, x, s), expected);
        }
    }
}
