// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/7/ITP1_7_B

fn run(r: usize, c: usize, a: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();

    for r in a.iter() {
        let mut v = Vec::new();

        for n in r.iter() {
            v.push(*n);
        }

        v.push(r.iter().sum::<usize>());

        ans.push(v);
    }

    ans.push(Vec::new());

    let mut total = 0;

    for i in 0..c {
        let mut sum = 0;

        for j in 0..r {
            sum += ans[j][i];
        }

        total += sum;
        ans[r].push(sum);
    }

    ans[r].push(total);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, Vec<Vec<usize>>);

    #[test]
    fn itp1_7_c() {
        let tests = [
            TestCase(4, 5, vec![vec![1, 1, 3, 4, 5], vec![2, 2, 2, 4, 5], vec![3, 3, 0, 1, 1], vec![2, 3, 4, 4, 6]], vec![vec![1, 1, 3, 4, 5, 14], vec![2, 2, 2, 4, 5, 15], vec![3, 3, 0, 1, 1, 8], vec![2, 3, 4, 4, 6, 19], vec![8, 9, 9, 13, 17, 56]]),
        ];

        for TestCase(r, c, a, expected) in tests {
            assert_eq!(run(r, c, a), expected);
        }
    }
}
