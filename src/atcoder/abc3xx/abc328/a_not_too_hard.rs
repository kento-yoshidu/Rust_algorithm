// https://atcoder.jp/contests/abc328/tasks/abc328_a

pub fn run(_n: usize, x: usize, s: Vec<usize>) -> usize {
    s.iter()
        .filter(|num| **num <= x)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(499, run(6, 200, vec![100, 675, 201, 200, 199, 328]));
        assert_eq!(5400, run(8, 675, vec![675, 675, 675, 675, 675, 675, 675, 675]));
        assert_eq!(0, run(8, 674, vec![675, 675, 675, 675, 675, 675, 675, 675]));
    }
}
