// https://atcoder.jp/contests/abc125/tasks/abc125_b

fn run(n: usize, v: Vec<isize>, c: Vec<isize>) -> isize {
    let mut ans = 0;

    for bit in 1..(1 << n) {
        let mut total = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                total += v[i] - c[i]
            }
        }

        ans = ans.max(total)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(3, vec![10, 2, 5], vec![6, 3, 4]));
        assert_eq!(6, run(4, vec![13, 21, 6, 19], vec![11, 30, 6, 15]));
        assert_eq!(0, run(1, vec![1], vec![50]));
    }
}
