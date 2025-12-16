// https://yukicoder.me/problems/no/799

fn run(a: usize, b: usize, c: usize, d: usize) -> usize {
    let mut ans = 0;

    for i in a..=b {
        for j in c..=d {
            if i != j {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn yuki_799() {
        let tests = [
            TestCase(1, 3, 2, 4, 7),
            TestCase(1, 5, 6, 10, 25),
            TestCase(1, 100, 1, 100, 9900),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
