// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_d

pub fn run(_n: usize, a: Vec<usize>, _m: usize, b: Vec<usize>) -> usize {
    a.iter().fold(0, |mut score, num| {
        score += num;

        if b.contains(&score) {
            0
        } else {
            score
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, run(4, vec![3, 1, 4, 1], 4, vec![2, 7, 1, 8]));
        assert_eq!(6, run(5, vec![1, 4, 1, 4, 2], 3, vec![1, 3, 5]));
        assert_eq!(20, run(2, vec![10, 10], 3, vec![1, 11, 11]));
    }
}
