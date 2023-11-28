// https://atcoder.jp/contests/abc203/tasks/abc203_b

pub fn run(n: usize, k: usize) -> usize {
    let mut ans = 0;

    for nn in 1..=n {
        for kk in 1..=k {
            ans += nn*100 + kk
        }
    }

    ans
}

pub fn run2(n :usize, k: usize) -> usize {
    (1..=k)
        .map(|l|  {
            (1..=n)
                .map(|m| {
                    m * 100 + l
            })
            .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(203, run(1, 2));
        assert_eq!(1818, run(3, 3));
    }

    #[test]
    fn test2() {
        assert_eq!(203, run2(1, 2));
        assert_eq!(1818, run2(3, 3));
    }
}
