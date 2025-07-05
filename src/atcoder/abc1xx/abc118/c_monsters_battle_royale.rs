// https://atcoder.jp/contests/abc118/tasks/abc118_c

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn run(_n: usize, v: Vec<usize>) -> usize {
    v.iter()
        .fold(0, |state, num| {
            gcd(state, *num)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc118_c() {
        let tests = [
            TestCase(4, vec![2, 10, 8, 40], 2),
            TestCase(4, vec![5, 13, 8, 1000000000], 1),
            TestCase(3, vec![1000000000, 1000000000, 1000000000], 1000000000),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
