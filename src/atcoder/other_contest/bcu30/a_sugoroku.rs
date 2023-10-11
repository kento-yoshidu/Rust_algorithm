// https://atcoder.jp/contests/bcu30/tasks/bcu30_a

pub fn run(n: usize, _k: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    for num in a.iter() {
        ans += num;

        if ans == n {
            return n;
        }

        if ans > n {
            ans = n - (ans - n);
        }
    };

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(10, 4, vec![5, 7, 2, 5]));
        assert_eq!(6, run(10, 4, vec![5, 7, 3 ,5]));
        assert_eq!(1, run(20, 7 , vec![12, 5, 5, 15, 2, 10, 20]));
    }
}
