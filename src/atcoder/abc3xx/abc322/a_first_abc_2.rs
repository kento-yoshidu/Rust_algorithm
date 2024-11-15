// https://atcoder.jp/contests/abc322/tasks/abc322_a

fn run(n: usize, s: &str) -> isize {
    if !s.contains("ABC") {
        return -1;
    }

    let vec: Vec<char> = s.chars().collect();

    for (i, index) in (0..n-2).enumerate() {
        if vec[i] == 'A' && vec[i+1] == 'B' && vec[i+2] == 'C' {
            return (index + 1) as isize;
        }
    }

    unreachable!()
}

fn run2(_n: usize, s: &str) -> isize {
    (s.find("ABC").unwrap_or(!0-1) + 1) as isize
}

fn run3(_n: usize, s: &str) -> isize {
    s.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .position(|v| {
            v == ['A', 'B', 'C']
        })
        .unwrap_or(!0-1) as isize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, "ABABCABC", 3),
            TestCase(8, "ACB", -1),
            TestCase(20, "BBAAABBACAACABCBABAB", 13),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
            assert_eq!(run3(n, s), expected);
        }
    }
}
