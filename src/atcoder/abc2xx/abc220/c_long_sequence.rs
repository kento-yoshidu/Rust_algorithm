// https://atcoder.jp/contests/abc220/tasks/abc220_c

pub fn run(n: usize, a: Vec<usize>, x: usize) -> usize {
    let sum: usize = a.iter().sum();

    let mut ans = (x / sum) * n;
    let mut rest = x % sum;

    let mut i = 0;

    loop {
        if rest < a[i] {
            return ans + 1
        } else {
            ans += 1;
            rest -= a[i];
            i = (i + 1) % n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, run(3, vec![3, 5, 2], 26));
        assert_eq!(23, run(4, vec![12, 34, 56, 78], 1000));
    }
}
