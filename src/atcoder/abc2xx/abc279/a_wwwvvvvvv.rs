// https://atcoder.jp/contests/abc279/tasks/abc279_a

fn run(str: &str) -> usize {
    let mut count = 0;

    for c in str.chars() {
        if c == 'w' {
            count = count + 2;
        } else {
            count = count + 1;
        }
    }

    count
}

fn run2(str: &str) -> usize {
    str.chars()
        .map(|c| {
            if c == 'w' {
                2
            } else {
                1
            }
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("vvwvw", 7),
            TestCase("v", 1),
            TestCase("wwwvvvvvv", 12),
            TestCase("wwvvvwwwwvvwwwwwwwvwvvvwwvvvvvwwwwvwwvwvvwwwvvvwvvwvwwwvwwvvwvwwvvvvwvwwvvwwwvvwwwvwvwwwwwwvwvwvvvwv", 155),
            TestCase("wwwwvvwvvwwvvvvwvvvwvvwvvwwwvwvvwvvwwwvvvvwvvwwvwvwvwwvvwwvvvvvvwvwwwwvwwvwwvvwvvvwvvwwwwvvwvwwvvwvw", 147),
            TestCase("vwwvvwvvvwwvvwvwvwwvvwvvvvwvwvwvwvwvwwvvvvwvwwwvwwvwvwwwvwvvvvwvwvwwvvvwwvvvwvwvwwvvvvwvvvwvwwwvvwvv", 144),
            TestCase("vwvwvvvvwvvwwvwwwwvvvw", 32),
            TestCase("wwwwvvwvvwwvvvvwvvvwvvwvvwwwvwvvwvvwwwvvvvwvvwwvwvwvwwvvwwvvvvvvwvwwwwvwwvwwvvwvvvwvvwwwwvvwvwwvvwvw", 147),
            TestCase("wwvvvvwvwvwvwvvvwvvwv", 29),
            TestCase("vwvwvwwwwv", 16),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
