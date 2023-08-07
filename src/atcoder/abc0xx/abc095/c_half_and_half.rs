// https://atcoder.jp/contests/abc095/tasks/abc095_c

#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32, x: i32, y: i32) -> i32 {
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

    #[test]
    fn test() {
        assert_eq!(7900, run(1500, 2000, 1600, 3, 2));
        assert_eq!(8500, run(1500, 2000, 1900, 3, 2));
        assert_eq!(100000000, run(1500, 2000, 500, 90000, 100000));
    }
}
