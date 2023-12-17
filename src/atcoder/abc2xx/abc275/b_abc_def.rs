// https://atcoder.jp/contests/abc275/tasks/abc275_b

fn run(a: i128, b: i128, c: i128, d: i128, e: i128, f: i128) -> i128 {
    let m = 998244353;

    let x = (a % m) * (b % m) * (c % m);
    let y = (d % m) * (e % m) * (f % m);

    (x + m - y) % m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(22, run(2, 3, 5, 1, 2, 4));
        assert_eq!(1755647, run(1, 1, 1000000000, 0, 0, 0));
        assert_eq!(0, run(1000000000000000000, 1000000000000000000, 1000000000000000000, 1000000000000000000, 1000000000000000000, 1000000000000000000));
        assert_eq!(998244352, run(998244353, 998244353, 998244353, 1, 1, 1));
    }
}
