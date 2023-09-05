// https://atcoder.jp/contests/abc021/tasks/abc021_a

pub fn run(n: usize) -> Vec<usize> {
    let mut ans = vec![n];

    for i in 0..4 {
        if n & 1 << i != 0 {
            ans.push(1 << i)
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![10, 2, 8], run(10));
        assert_eq!(vec![9, 1, 8], run(9));
        assert_eq!(vec![8, 8], run(8));
        assert_eq!(vec![7, 1, 2, 4], run(7));
        assert_eq!(vec![6, 2, 4], run(6));
        assert_eq!(vec![5, 1, 4], run(5));
        assert_eq!(vec![4, 4], run(4));
        assert_eq!(vec![3, 1, 2], run(3));
        assert_eq!(vec![2, 2], run(2));
        assert_eq!(vec![1, 1], run(1));
    }
}