// https://atcoder.jp/contests/abc095/tasks/abc095_c

fn run(a: usize, b: usize, c: usize, x: usize, y: usize) -> usize {
    // ABの方が高いならAとBを単品購入
    if a + b <= c*2 {
        return a*x + b*y
    }

    let mut ans = 0;

    // 少ない方の枚数
    let min = x.min(y);

    ans += c*2 * min;

    if x > y {
        ans += ((x - min) * a).min((x-min) * c * 2)
    } else {
        ans += ((y - min) * b).min((y-min) * c * 2)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1500, 2000, 1600, 3, 2, 7900),
            TestCase(1500, 2000, 1900, 3, 2, 8500),
            TestCase(1500, 2000, 500, 90000, 100000, 100000000),
        ];

        for TestCase(a, b, c, x, y, expected) in tests {
            assert_eq!(run(a, b, c, x, y), expected);
        }
    }
}
