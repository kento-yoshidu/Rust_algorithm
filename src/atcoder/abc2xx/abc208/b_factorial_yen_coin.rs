// https://atcoder.jp/contests/abc206/tasks/abc206_b

fn fact(num: usize) -> usize {
    let mut result = 1;

    for i in 1..=num {
        result = result * i;
    }

    result
}

fn run(n: usize) -> usize {
    let mut total = n;
    let mut result = 0;

    for i in (1..=10).rev() {
        while total >= fact(i) {
            total = total - fact(i);
            result = result + 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc208_b() {
        let tests = [
            TestCase(9, 3),
            TestCase(119, 10),
            TestCase(10000000, 24),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
