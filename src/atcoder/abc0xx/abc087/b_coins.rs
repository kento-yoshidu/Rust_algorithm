// https://atcoder.jp/contests/abc087/tasks/abc087_b

pub fn run(a: isize, b: isize, c: isize, x: isize) -> usize {
    let mut ans = 0;

    for i in 0..=a {
        for j in 0..=b {
            if i*500 + j*100 > x {
                continue;
            }

            if i* 500 + j*100 == x {
                ans += 1;
                continue;
            }

            let rest = (x - (i*500 + j*100)) / 50;

            if rest <= c {
                ans += 1;
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
        assert_eq!(2, run(2, 2, 2, 100));
        assert_eq!(0, run(5, 1, 0, 150));
        assert_eq!(213, run(30, 40, 50, 6000));
    }
}
