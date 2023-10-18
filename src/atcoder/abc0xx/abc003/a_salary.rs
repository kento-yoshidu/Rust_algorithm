// https://atcoder.jp/contests/abc003/tasks/abc003_1

pub fn run(n: usize) -> usize {
    let mut sum = 0_f64;

    for i in 1..=n {
        sum += i as f64 * 10000_f64;
    }

    sum as usize / n
}

pub fn run2(n: usize) -> usize {
    (1..=n).map(|num| {
        num * 10000
    }).sum::<usize>() / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(35000, run(6));
        assert_eq!(460000, run(91));
    }

    #[test]
    fn test2() {
        assert_eq!(35000, run2(6));
        assert_eq!(460000, run2(91));
    }
}
