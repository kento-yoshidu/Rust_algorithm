// https://atcoder.jp/contests/abc227/tasks/abc227_a

pub fn run(n: usize, k: usize, a: usize) -> usize {
    let ans = (k + a - 1) % n;

    if ans != 0 {
        ans
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 3, 2));
        assert_eq!(1, run(1, 100, 1));
        assert_eq!(3, run(3, 14, 2));
    }
}
