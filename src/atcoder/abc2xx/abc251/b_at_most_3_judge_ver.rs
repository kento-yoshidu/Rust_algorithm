// https://atcoder.jp/contests/abc251/tasks/abc251_b

pub fn run(n: usize, w: usize, a: Vec<usize>) -> usize {
    let mut ans = vec![false; w];

    for i in 0..n {
        if a[i] <= w {
            ans[a[i]-1] = true;
        }
    }

    for i in 0..n {
        for j in i+1..n {
            if a[i] + a[j] <= w {
                ans[(a[i] + a[j])-1] = true;
            }
        }
    }


    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if a[i] + a[j] + a[k] <= w {
                    ans[(a[i] + a[j] + a[k])-1] = true;
                }
            }
        }
    }

    ans.iter()
        .filter(|e| **e == true)
        .count()
}

pub fn run2(_n: usize, w: usize, a: Vec<usize>) -> usize {
    use itertools::Itertools;

    let mut ans = vec![false; w];

    for i in 0..3 {
        a.iter()
            .combinations(i+1)
            .for_each(|t| {
                let sum = t.into_iter().sum::<usize>();

                if sum <= w {
                    ans[sum-1] = true
                }
            });
    }

    ans.iter()
        .filter(|e| **e == true)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(2, 10, vec![1, 3]));
        assert_eq!(0, run(2, 1, vec![2, 3]));
        assert_eq!(3, run(4, 12, vec![3, 3, 3, 3]));
        assert_eq!(48, run(7, 251, vec![202, 20, 5, 1, 4, 2, 100]));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2(2, 10, vec![1, 3]));
        assert_eq!(0, run2(2, 1, vec![2, 3]));
        assert_eq!(3, run2(4, 12, vec![3, 3, 3, 3]));
        assert_eq!(48, run2(7, 251, vec![202, 20, 5, 1, 4, 2, 100]));
    }
}
