// https://yukicoder.me/problems/no/32

fn run(l: usize, m: usize, n: usize) -> usize {
    let mut n = n;
    let mut m = m;
    let mut l = l;

    m += n / 25;
    n %= 25;

    l += m / 4;
    m %= 4;

    l %= 10;

    n + m + l
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn yuki_032() {
        let tests = [
            TestCase(7, 20, 10, 12),
            TestCase(0, 0, 0, 0),
        ];

        for TestCase(l, m, n, expected) in tests {
            assert_eq!(run(l, m, n), expected);
        }
    }
}
