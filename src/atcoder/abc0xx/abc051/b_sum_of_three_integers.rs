// https://atcoder.jp/contests/abc051/tasks/abc051_b

pub fn run(k: usize, s: usize) -> usize {
    let mut ans = 0;

    for x in 0..=k {
        for y in 0..=k {
            if x + y > s {
                break
            }

            if x + y == s {
                ans += 1;
                break
            }

            if (s - x - y) <= k {
                ans += 1
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, run(2, 2));
        assert_eq!(1, run(5, 15));
    }
}
