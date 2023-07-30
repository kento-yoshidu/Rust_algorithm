// https://atcoder.jp/contests/abc003/tasks/abc003_1

#[allow(dead_code)]
pub fn run(n: usize) -> usize {
    let mut sum = 0_f64;

    for i in 1..=n {
        sum += i as f64 * 10000_f64;
    }

    sum as usize / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(35000, run(6));
        assert_eq!(460000, run(91));
    }
}
