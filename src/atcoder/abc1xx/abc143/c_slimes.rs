// https://atcoder.jp/contests/abc143/tasks/abc143_c

fn run(_n: usize, s: &str) -> usize {
    let mut chars: Vec<char> = s.chars().collect();

    chars.dedup();

    chars.len()
}

fn run_lengths(s: Vec<char>) -> Vec<(char, usize)> {
    let mut run_lengths = vec![];
    let mut current = (s[0], 1);

    for i in 1..s.len() {
        if s[i] == current.0 {
            current.1 += 1;
        } else {
            run_lengths.push(current);
            current = (s[i], 1);
        }
    }

    run_lengths.push(current);

    run_lengths
}

fn run2(_n: usize, s: &str) -> usize {
    run_lengths(s.chars().collect()).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn abc143_c() {
        let tests = [
            TestCase(10, "aabbbbaaca", 5),
            TestCase(5, "aaaaa", 1),
            TestCase(20, "xxzaffeeeeddfkkkkllq", 10),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
