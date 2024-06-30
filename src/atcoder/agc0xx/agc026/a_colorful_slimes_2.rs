// https://atcoder.jp/contests/agc026/tasks/agc026_a

fn run_length(s: Vec<usize>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut current = (s[0], 1);

    for i in 1..s.len() {
        if s[i] == current.0 {
            current.1 += 1;
        } else {
            result.push(current);
            current = (s[i], 1);
        }
    }

    result.push(current);

    result
}

fn run(n: usize, a: Vec<usize>) -> usize {
    run_length(a)
        .into_iter()
        .map(|(_, num)| num / 2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 1, 2, 2, 2], 2),
            TestCase(3, vec![1, 2, 1], 0),
            TestCase(5, vec![1, 1, 1, 1, 1], 2),
            TestCase(14, vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 1, 2, 3, 4], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}