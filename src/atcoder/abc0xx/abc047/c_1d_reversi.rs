// https://atcoder.jp/contests/abc047/tasks/arc063_a

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

fn run(s: &str) -> usize {
    run_lengths(s.chars().collect()).len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("BBBWW", 1),
            TestCase("WWWWWW", 0),
            TestCase("WBWBWBWBWB", 9),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
