// https://yukicoder.me/problems/no/3295

fn run(n: usize) -> usize {
    if n <= 99 {
        0
    } else {
        (n - 99 + 499) / 500
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn yuki_3295() {
        let tests = [
            TestCase(700, 2),
            TestCase(550, 1),
            TestCase(2300, 5),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
