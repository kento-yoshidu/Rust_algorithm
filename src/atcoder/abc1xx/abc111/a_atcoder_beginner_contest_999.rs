// https://atcoder.jp/contests/abc111/tasks/abc111_a

fn run(n: usize) -> usize {
    n.to_string()
        .chars()
        .map(|c| {
            match c {
                '1' => '9',
                _ => '1'
            }
        })
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn run2(n: usize) -> usize {
    1110 - n
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc111_a() {
        let tests = [
            TestCase(119, 991),
            TestCase(999, 111),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
