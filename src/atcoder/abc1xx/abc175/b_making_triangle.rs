// https://atcoder.jp/contests/abc175/tasks/abc175_b

use itertools::Itertools;

fn run(n: usize, l: &Vec<usize>) -> usize {
    let mut vec = l.clone();

    vec.sort();

    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            if vec[i] == vec[j] {
                continue;
            }

            for k in j..n {
                if vec[j] == vec[k] {
                    continue;
                }

                if vec[i] + vec[j] > vec[k] {
                    ans += 1;
                }
            }
        }
    }

    ans
}

fn run2(_n: usize, l: &Vec<usize>) -> usize {
    let mut vec = l.clone();

    vec.sort();

    vec.into_iter()
        .combinations(3)
        .filter(|t| {
            (t[0] != t[1] && t[1] != t[2] && t[0] != t[2]) && (t[0] + t[1] > t[2])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc175_b() {
        let tests = [
            TestCase(5, vec![4, 4, 9, 7, 5], 5),
            TestCase(6, vec![4, 5, 4, 3, 3, 5], 8),
            TestCase(10, vec![9, 4, 6, 1, 9, 6, 10, 6, 6, 8], 39),
            TestCase(2, vec![1, 1], 0),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, &l), expected);
            assert_eq!(run2(n, &l), expected);
        }
    }
}
