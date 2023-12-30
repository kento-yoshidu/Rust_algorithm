// https://atcoder.jp/contests/abc217/tasks/abc217_c

pub fn run(n: usize, p: Vec<usize>) -> Vec<usize> {
    let mut q = vec![0; n];

    for (i, num) in p.iter().enumerate() {
        q[num-1] = i+1;
    }

    q
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 1, 2], run(3, vec![2, 3, 1]));
        assert_eq!(vec![1, 2, 3], run(3, vec![1, 2, 3]));
        assert_eq!(vec![5, 3, 2, 4, 1], run(5, vec![5, 3, 2, 4, 1]));
    }
}
