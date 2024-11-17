// https://atcoder.jp/contests/abc380/tasks/abc380_c

fn run_length(s: Vec<char>) -> Vec<(char, usize)> {
    let mut result = vec![];
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

fn run(_n: usize, k: usize, s: &str) -> String {
    let mut rle = run_length(s.chars().collect());

    let one_idx = rle
        .iter()
        .enumerate()
        .filter(|&(_, &(ch, _))| ch == '1')
        .nth(k - 1)
        .map(|(i, _)| i)
        .unwrap();

    if one_idx > 0 && rle[one_idx - 1].0 == '0' {
        rle.swap(one_idx - 1, one_idx);
    }

    rle
        .iter()
        .flat_map(|&(ch, len)| std::iter::repeat(ch).take(len))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(15, 3, "010011100011001", "010011111000001"),
            TestCase(10, 2, "1011111111", "1111111110"),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
