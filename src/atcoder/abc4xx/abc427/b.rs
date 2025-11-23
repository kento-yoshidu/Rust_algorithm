// https://atcoder.jp/contests/abc427/tasks/abc427_b

fn calc(mut x: usize) -> usize {
    let mut s = 0;

    while x > 0 {
        s += x % 10;
        x /= 10;
    }

    s
}

fn run(n: usize) -> usize {
    let mut a = 1;

    for _ in 0..n-1 {
        a += calc(a);
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc427_b() {
        let tests = [
            TestCase(6, 23),
            TestCase(45, 427),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
