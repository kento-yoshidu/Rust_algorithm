// https://atcoder.jp/contests/abc111/tasks/abc111_a

fn run(n: usize) -> usize {
    let mut result = String::new();

    n.to_string()
        .chars()
        .for_each(|i| {
            if i == '9' {
                result.push('1')
            } else {
                result.push('9')
            }
        });

    result.parse::<usize>().unwrap()
}

fn run2(n: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
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
