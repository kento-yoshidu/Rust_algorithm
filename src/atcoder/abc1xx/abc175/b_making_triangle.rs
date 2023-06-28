// https://atcoder.jp/contests/abc175/tasks/abc175_b

#[allow(dead_code)]
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
}
