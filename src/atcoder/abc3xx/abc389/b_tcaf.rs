// https://atcoder.jp/contests/abc389/tasks/abc389_b

fn rec(n: usize) -> usize {
    if n == 1 {
        n
    } else {
        n * rec(n-1)
    }
}

fn run(x: usize) -> usize {
    for i in 1..=20 {
        let num = rec(i);

        if x == num {
            return i;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 3),
            TestCase(2432902008176640000, 20),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
