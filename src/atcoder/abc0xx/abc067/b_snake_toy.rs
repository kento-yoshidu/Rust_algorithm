// https://atcoder.jp/contests/abc067/tasks/abc067_a

pub fn run(_n: i32, k: usize, vec: &mut Vec<i32>) -> i32 {
    vec.sort();

    vec.iter().rev().take(k).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(12, run(5, 3, &mut vec![1, 2, 3, 4, 5]));
        assert_eq!(386, run(15, 14, &mut vec![50, 26, 27, 21, 41, 7, 42, 35, 7, 5, 5, 36, 39, 1, 45]));
    }
}
