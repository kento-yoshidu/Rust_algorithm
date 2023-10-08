// https://atcoder.jp/contests/abc187/tasks/abc187_a

pub fn run(a: i32, b: i32) -> i32 {
    // 百の位、十の位、一の位
    let total_a = (a / 100) + (a % 100 / 10) + (a % 10);
    let total_b = (b / 100) + (b % 100 / 10) + (b % 10);

    if a > b {
        total_a
    } else {
        total_b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(123, 234));
        assert_eq!(17, run(593, 953));
        assert_eq!(27, run(100, 999));
    }
}
