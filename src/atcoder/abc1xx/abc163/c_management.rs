// https://atcoder.jp/contests/abc163/tasks/abc163_c

pub fn run(n: usize, aa: Vec<usize>) -> Vec<usize> {
    let mut ans = vec![0; n];

    for i in aa {
        ans[i-1] += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 2, 0, 0, 0], run(5, vec![1, 1, 2, 2]));
        assert_eq!(vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0], run(10, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]));
        assert_eq!(vec![1, 1, 1, 1, 1, 1, 0], run(7, vec![1, 2, 3, 4, 5, 6]));
    }
}
