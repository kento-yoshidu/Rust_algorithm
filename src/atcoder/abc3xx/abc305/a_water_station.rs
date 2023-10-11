// https://atcoder.jp/contests/abc305/tasks/abc305_a

pub fn run(n: usize) -> usize {
    let station = n / 5 * 5;

    if n - station < 3 {
        station
    } else {
        station + 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(55, run(53));
        assert_eq!(20, run(21));
        assert_eq!(100, run(100));
    }
}
