// https://atcoder.jp/contests/abc081/tasks/abc081_a

fn run(str: &str) -> usize {
    let mut count = 0;

    for c in str.chars() {
        if c == '1' {
            count += 1;
        }
    }

    count
}

fn run2(str: &str) -> usize {
    str.chars()
        .filter(|c| { *c == '1' })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("101", 2),
            TestCase("000", 0),
        ];

        for TestCase(str, expected) in tests {
            assert_eq!(run(str), expected);
            assert_eq!(run2(str), expected);
        }
    }
}
