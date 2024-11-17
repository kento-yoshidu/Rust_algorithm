// https://atcoder.jp/contests/abc380/tasks/abc380_b

fn run(s: &str) -> Vec<usize> {
    let mut ans = Vec::new();

    let mut state = 0;

    for c in s.chars() {
        match c {
            '|' => {
                if state != 0 {
                    ans.push(state);
                }

                state = 0;
            },
            '-' => {
                state += 1;
            }
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase("|---|-|----|-|-----|", vec![3, 1, 4, 1, 5]),
            TestCase("|----------|", vec![10]),
            TestCase("|-|-|-|------|", vec![1, 1, 1, 6]),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
