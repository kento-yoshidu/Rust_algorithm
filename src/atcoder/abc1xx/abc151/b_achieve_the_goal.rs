// https://atcoder.jp/contests/abc151/tasks/abc151_b

#[allow(dead_code)]
pub fn run(n: i32, k: i32, m: i32, vec: Vec<i32>) -> i32 {
    let sum: i32 = vec.iter().sum();

    let line = n * m;

    if (line - sum) > k {
        return -1;
    } else if (line - sum) < 0 {
        return 0;
    } else {
        return line - sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(5, 10, 7, vec![8, 10, 3, 6]));
        assert_eq!(0, run(4, 100, 60, vec![100, 100, 100]));
        assert_eq!(-1, run(4, 100, 60, vec![0, 0, 0]));
    }
}
