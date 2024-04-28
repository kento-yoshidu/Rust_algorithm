// https://atcoder.jp/contests/soundhound2018-summer-qual/tasks/soundhound2018_summer_qual_a

fn run(a: usize, b: usize) -> char {
    if a + b == 15 {
        '+'
    } else if a * b == 15 {
        '*'
    } else {
        'x'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, char);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 11, '+'),
            TestCase(3, 5, '*'),
            TestCase(1, 1, 'x'),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
