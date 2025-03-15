// https://atcoder.jp/contests/abc039/tasks/abc039_b

fn run(n: usize) -> usize {
    for i in 1..=1000 {
        if i*i*i*i == n {
            return i;
        }
    }

    unreachable!();
}

fn run2(n: usize) -> usize {
    ((n as f64).sqrt()).sqrt() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 1),
            TestCase(390625, 25),
            TestCase(981506241, 177),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
