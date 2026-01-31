// https://atcoder.jp/contests/abc243/tasks/abc243_a

fn run(v: usize, a: usize, b: usize, c: usize) -> char {
    let rest = v % (a + b + c);

    if rest < a {
        'F'
    } else if rest < (a + b) {
        'M'
    } else {
        'T'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, char);

    #[test]
    fn abc243_a() {
        let tests = [
            TestCase(25, 10, 11, 12, 'T'),
            TestCase(30, 10, 10, 10, 'F'),
            TestCase(100000, 1, 1, 1, 'M'),
        ];

        for TestCase(v, a, b, c, expected) in tests {
            assert_eq!(run(v, a, b, c), expected);
        }
    }
}
