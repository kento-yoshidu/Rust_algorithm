// https://atcoder.jp/contests/abc163/tasks/abc163_b

pub fn run(n: usize, _m: usize, a: Vec<usize>) -> isize {
    let sum = a.iter().sum();

    if n >= sum {
        (n - sum) as isize
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(30, run(41, 2, vec![5, 6]));
        assert_eq!(-1, run(10, 2, vec![5, 6]));
        assert_eq!(0, run(11, 2, vec![5, 6]));
        assert_eq!(9, run(314, 15, vec![9, 26, 5, 35, 8, 9, 79, 3, 23, 8, 46, 2, 6, 43, 3]));
    }
}
