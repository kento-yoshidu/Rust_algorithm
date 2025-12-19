// https://atcoder.jp/contests/abc205/tasks/abc205_c

fn run(a: isize, b: isize, c: isize) -> char {
    if c % 2 == 0 {
        if a.abs() == b.abs() {
            '='
        } else if a.abs() > b.abs() {
            '>'
        } else {
            '<'
        }
    } else {
        if a > b {
            '>'
        } else if a < b {
            '<'
        } else {
            '='
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, char);

    #[test]
    fn abc205_c() {
        let tests = [
            TestCase(3, 2, 4, '>'),
            TestCase(-7, 7, 2, '='),
            TestCase(-8, 3, 3, '<'),
            TestCase(796382932, -905246003, 182548924, '<'),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
