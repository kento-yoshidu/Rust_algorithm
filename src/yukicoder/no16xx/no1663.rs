// https://yukicoder.me/problems/no/1663

fn run(a: usize, b: usize, c: usize, d: usize, m: usize) -> usize {
    let mut ans = 0;

    for i in a..=b {
        for j in c..=d {
            ans = ans.max((i + j) % m);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize);

    #[test]
    fn yuki_1663() {
        let tests = [
            TestCase(2, 4, 3, 5, 7, 6),
            TestCase(1, 1, 5, 6, 9, 7),
        ];

        for TestCase(a, b,  c, d, m, expected) in tests {
            assert_eq!(run(a, b, c, d, m), expected);
        }
    }
}
