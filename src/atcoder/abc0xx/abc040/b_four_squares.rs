// https://atcoder.jp/contests/abc040/tasks/abc040_b

pub fn run(n: isize) -> isize {
    let mut ans = std::isize::MAX;

    for i in 1..n {
        let amari = n % i;

        let tmp = amari + (i - (n / i)).abs();

        ans = ans.min(tmp);
    }

    ans
}

pub fn run2(n: isize) -> isize {
    (1..n)
        .map(|i| {
            let amari = n % i;

            amari + (i - (n / i)).abs()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(26));
        assert_eq!(4, run(41));
        assert_eq!(37, run(100000));
    }
}
