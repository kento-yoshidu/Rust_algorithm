// https://atcoder.jp/contests/abc068/tasks/abc068_b

fn calc(n: usize, count: usize) -> usize {
    if n % 2 != 0 {
        return count
    } else {
        calc(n / 2, count+1)
    }
}

fn run(n: usize) -> usize {
    (1..=n)
        .skip(1)
        .step_by(2)
        .map(|i| {
            (i, calc(i, 0))
        })
        .filter(|t| {
            t.1 != 0
        })
        .max_by_key(|t| {
            t.1
        })
        .unwrap_or((1, 0)).0
}

fn calc2(n: usize) -> bool {
    if n / 2 == 0 {
        return true
    }

    if n % 2 != 0 {
        return false
    }

    calc2(n / 2)
}

fn run2(n: usize) -> usize {
    (1..=n)
        .rev()
        .find(|num| {
            calc2(*num)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 4),
            TestCase(32, 32),
            TestCase(1, 1),
            TestCase(100, 64),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
