// https://atcoder.jp/contests/abc010/tasks/abc010_2

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in a {
        let mut num = i;

        loop {
            if num % 2 == 0 {
                ans += 1;
                num -= 1;
            } else if num % 3 == 2 {
                ans += 1;
                num -= 1;
            } else {
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![5, 8, 2], 4),
            TestCase(9, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 8),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
