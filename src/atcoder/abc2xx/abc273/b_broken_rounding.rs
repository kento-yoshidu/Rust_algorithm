// https://atcoder.jp/contests/abc273/tasks/abc273_b

fn calc(n: usize, k: u32) -> usize {
    if n % 10_usize.pow(k) < 5 * 10_usize.pow(k-1) {
        n - (n % 10_usize.pow(k))
    } else {
        (10_usize.pow(k) - n % 10_usize.pow(k)) + n
    }
}

pub fn run(x: usize, k: u32) -> usize {
    (1..=k)
        .fold(x, |state, k| {
            calc(state, k)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2100, run(2048, 2));
        assert_eq!(0, run(1, 15));
        assert_eq!(1000, run(999, 3));
        assert_eq!(314000000000000, run(314159265358979, 12));
    }
}
