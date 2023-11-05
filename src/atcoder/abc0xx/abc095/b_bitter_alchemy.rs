// https://atcoder.jp/contests/abc095/tasks/abc095_b

pub fn run(n: i32, x: i32, vec: Vec<i32>) -> i32 {
    let minimum: i32 = vec.iter().sum();

    let rest = x - minimum;

    let mut ans = n;

    if rest != 0 {
        let min = vec.iter().min().unwrap();

        ans += rest / min;
    }

    ans
}

pub fn run2(n: usize, x: usize, m: Vec<usize>) -> usize {
    let min = m.iter().min().unwrap();
    let sum: usize = m.iter().sum();
    let rest = x - sum;

    n + rest / min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(3, 1000, vec![120, 100, 140]));
        assert_eq!(4, run(4, 360, vec![90, 90, 90, 90]));
        assert_eq!(26, run(5, 3000, vec![150, 130, 150, 130, 110]));
    }

    #[test]
    fn test2() {
        assert_eq!(9, run2(3, 1000, vec![120, 100, 140]));
        assert_eq!(4, run2(4, 360, vec![90, 90, 90, 90]));
        assert_eq!(26, run2(5, 3000, vec![150, 130, 150, 130, 110]));
    }
}
