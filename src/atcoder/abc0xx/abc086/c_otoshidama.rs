// https://atcoder.jp/contests/abc085/tasks/abc085_c

#[allow(dead_code)]
pub fn run(n: isize, x: isize) -> Vec<isize> {
    for a in 0..=n {
        for b in 0..=n {
            // 千円札がc枚だったらx円になる
            let c = (x - (a*10000 + b*5000)) / 1000;

            if c >= 0 && n == (a + b + c) {
                return vec![a, b, c];
            }
        }
    }

    vec![-1, -1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 9, 0], run(9, 45000));
        assert_eq!(vec![1, 7, 1], run(9, 46000));
        assert_eq!(vec![-1, -1, -1], run(20, 196000));
    }
}
