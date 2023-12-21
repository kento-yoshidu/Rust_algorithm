// https://atcoder.jp/contests/abc121/tasks/abc121_c

pub fn run(_n: usize, m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut vec = ab.clone();

    vec.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ans = 0;
    let mut count = 0;

    for (a, b) in vec {
        if count >= m {
            return ans;
        }

        if count + b >= m {
            return ans + (m - count) * a;
        } else {
            ans += a*b;
            count += b;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(12, run(2, 5, vec![(4, 9), (2, 4)]));
        assert_eq!(130, run(4, 30, vec![(6, 18), (2, 5), (3, 10), (7, 9)]));
        assert_eq!(100000000000000, run(1, 100000, vec![(1000000000, 100000)]));
    }
}
