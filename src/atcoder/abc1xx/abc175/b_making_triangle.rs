// https://atcoder.jp/contests/abc175/tasks/abc175_b

pub fn run(n: usize, mut vec: Vec<usize>) -> usize {
    let mut ans = 0;

    vec.sort();

    for i in 0..n {
        for j in i..n {
            if vec[i] == vec[j] {
                continue
            }

            for k in j..n {
                if vec[j] == vec[k] {
                    continue
                }

                if vec[i] + vec[j] > vec[k] {
                    ans += 1;
                }
            }
        }
    }

    ans
}

pub fn run2(_n: usize, l: Vec<usize>) -> usize {
    use itertools::Itertools;

    let mut vec = l.clone();

    vec.sort();

    vec.iter()
        .combinations(3)
        .filter(|t| {
            (t[0] != t[1] && t[1] != t[2] && t[0] != t[2]) && (t[0] + t[1] > *t[2])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(5, vec![4, 4, 9, 7, 5]));
        assert_eq!(8, run(6, vec![4, 5, 4, 3, 3, 5]));
        assert_eq!(39, run(10, vec![9, 4, 6, 1, 9, 6, 10, 6, 6, 8]));
        assert_eq!(0, run(2, vec![1, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(5, run2(5, vec![4, 4, 9, 7, 5]));
        assert_eq!(8, run2(6, vec![4, 5, 4, 3, 3, 5]));
        assert_eq!(39, run2(10, vec![9, 4, 6, 1, 9, 6, 10, 6, 6, 8]));
        assert_eq!(0, run2(2, vec![1, 1]));
    }
}
