// https://atcoder.jp/contests/abc259/tasks/abc259_a

pub fn run(_n: isize, m: isize, x: isize, t: isize, d: isize) -> isize {
    if x <= m {
        return t
    }

    t - (x - m) * d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(168, run(38, 20, 17, 168, 3));
        assert_eq!(1, run(1, 0, 1, 3, 2));
        assert_eq!(90, run(100, 10, 100, 180, 1));
        assert_eq!(199, run(100, 99, 100, 200, 1));
        assert_eq!(1, run(1, 0, 1, 2, 1));
        assert_eq!(121, run(64, 17, 45, 177, 2));
        assert_eq!(180, run(29, 5, 1, 180, 100));
        assert_eq!(80, run(100, 0, 30, 200, 4));
        assert_eq!(143, run(23, 15, 23, 199, 7));
        assert_eq!(108, run(53, 27, 2, 108, 50));
    }
}
