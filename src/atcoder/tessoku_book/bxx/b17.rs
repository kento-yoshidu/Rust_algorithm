// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cp


use std::cmp::min;

pub fn run(n: usize, h: Vec<isize>) -> (usize, Vec<usize>) {
    let mut dp = vec![0];
    dp.push((h[1] - h[0]).abs());

    for i in 2..n {
        dp.push(min(dp[i-2] + (h[i-2] - h[i]).abs(), dp[i-1] + (h[i] - h[i-1]).abs()));
    }

    (1, vec![0])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, (usize, Vec<usize>));
    #[test]
    fn test() {
        let tests= [
            TestCase(4, vec![10, 30, 40, 20], (3, vec![1, 2, 4])),
            TestCase(6, vec![30, 10, 60, 10, 60, 50], (4, vec![1, 3, 5, 6])),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
