// https://atcoder.jp/contests/abc100/tasks/abc100_b

pub fn run(d: usize, n: usize) -> usize {
    if n == 100 {
        (100_u32.pow(d as u32)) as usize * 101
    } else {
        (100_u32.pow(d as u32)) as usize * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(0, 5));
        assert_eq!(1100, run(1, 11));
        assert_eq!(850000, run(2, 85));
        assert_eq!(101, run(0, 100));
    }
}
