// https://atcoder.jp/contests/abc251/tasks/abc251_b

fn run(n: usize, w: usize, a: &Vec<usize>) -> usize {
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

    ans.into_iter()
        .filter(|e| *e == true)
        .count()
}

fn run2(_n: usize, w: usize, a: &Vec<usize>) -> usize {
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

    ans.into_iter()
        .filter(|e| *e == true)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc251_b() {
        let tests = [
            TestCase(2, 10, vec![1, 3], 3),
            TestCase(2, 1, vec![2, 3], 0),
            TestCase(4, 12, vec![3, 3, 3, 3], 3),
            TestCase(7, 251, vec![202, 20, 5, 1, 4, 2, 100], 48),
        ];

        for TestCase(n, w, a, expected) in tests {
            assert_eq!(run(n, w, &a), expected);
            assert_eq!(run2(n, w, &a), expected);
        }
    }
}
