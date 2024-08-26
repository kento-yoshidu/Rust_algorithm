// https://atcoder.jp/contests/abc363/tasks/abc363_a

fn run(r: usize) -> usize {
    match r {
        1..=99 => {
            100 - r
        },
        100..=199 => {
            200 - r
        },
        200..=299 => {
            300 - r
        },
        300..=399 => {
            400 - r
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(123, 77),
            TestCase(250, 50),
        ];

        for TestCase(r, expected) in tests {
            assert_eq!(run(r), expected);
        }
    }
}
