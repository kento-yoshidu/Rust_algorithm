// https://atcoder.jp/contests/arc113/tasks/arc113_a

fn run(k: usize) -> usize {
    let mut ans = 0;

    for i in 1..=k {
        for j in 1..=k {
            if i*j > k {
                break;
            }

            ans += k / (i*j);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 4),
            TestCase(10, 53),
            TestCase(31415, 1937281),
        ];

        for TestCase(k, expected) in tests {
            assert_eq!(run(k), expected);
        }
    }
}
