// https://atcoder.jp/contests/abc087/tasks/abc087_b

fn run(a: isize, b: isize, c: isize, x: isize) -> usize {
    let mut ans = 0;

    for i in 0..=a {
        for j in 0..=b {
            if i*500 + j*100 > x {
                continue;
            }

            if i* 500 + j*100 == x {
                ans += 1;
                continue;
            }

            let rest = (x - (i*500 + j*100)) / 50;

            if rest <= c {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 2, 100, 2),
            TestCase(5, 1, 0, 150, 0),
            TestCase(30, 40, 50, 6000, 213),
        ];

        for TestCase(a, b, c, x, expected) in tests {
            assert_eq!(run(a, b, c, x), expected);
        }
    }
}
