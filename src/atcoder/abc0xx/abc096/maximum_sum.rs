// https://atcoder.jp/contests/abc096/tasks/abc096_b

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(30, run(vec![5, 3, 11], 1));
        assert_eq!(22, run(vec![3, 3, 4], 2));
    }
}
