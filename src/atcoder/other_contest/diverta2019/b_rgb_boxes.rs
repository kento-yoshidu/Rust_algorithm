// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_b

pub fn run(r: usize, g: usize, b: usize, n: usize) -> usize {
    let mut ans = 0;

    for i in 0..=n {
        for j in 0..=n {
            if n < i*r + j*g {
                break;
            }

            if (n - (i*r + j*g)) % b == 0 {
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
    fn test() {
        let tests = [
            TestCase(1, 2, 3, 4, 4),
            TestCase(13, 1, 4, 3000, 87058),
        ];

        for TestCase(r, g, b, n, expected) in tests {
            assert_eq!(run(r, g, b, n), expected);
        }
    }
}
