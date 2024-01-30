// https://atcoder.jp/contests/abc248/tasks/abc248_b

fn calc(count: usize, a: usize, b: usize, k: usize) -> usize {
    if a >= b {
        count
    } else {
        calc(count+1, a*k, b, k)
    }
}

fn run(a: usize, b: usize, k: usize) -> usize {
    calc(0, a, b, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(1, 4, 2));
        assert_eq!(0, run(7, 7, 10));
        assert_eq!(6, run(31, 415926, 5));
        assert_eq!(1, run(158260522, 200224287, 10));
        assert_eq!(30, run(1, 1000000000, 2));
        assert_eq!(1, run(999999999, 1000000000, 500000000));
        assert_eq!(29, run(1, 536870912, 2));
    }
}
