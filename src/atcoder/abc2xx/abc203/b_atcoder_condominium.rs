// https://atcoder.jp/contests/abc203/tasks/abc203_b

fn run(n: usize, k: usize) -> usize {
    let mut ans = 0;

    for nn in 1..=n {
        for kk in 1..=k {
            ans += nn*100 + kk
        }
    }

    ans
}

fn run2(n: usize, k: usize) -> usize {
    (1..=k)
        .map(|l|  {
            (1..=n)
                .map(|m| {
                    m * 100 + l
            })
            .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc203_b() {
        let tests = [
            TestCase(1, 2, 203),
            TestCase(3, 3, 1818),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
            assert_eq!(run2(n, k), expected);
        }
    }
}
