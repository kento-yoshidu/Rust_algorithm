// https://atcoder.jp/contests/abc350/tasks/abc350_c

fn run(n: usize, a: Vec<usize>) -> (usize, Option<Vec<(usize, usize)>>) {
    let mut a: Vec<usize> = a.into_iter().map(|x| x-1).collect();
    let mut vec = vec![0; n];

    for i in 0..n {
        vec[a[i]] = i;
    }

    let mut ans = Vec::new();

    for i in 0..n {
        if i != a[i] {
            let j = vec[i];
            let x = a[i];

            a.swap(i, j);
            vec[x] = j;
            ans.push((i+1, j+1));
        }
    }

    if ans.is_empty() {
        (0, None)
    } else {
        (ans.len(), Some(ans))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, Option<Vec<(usize, usize)>>));

    #[test]
    fn abc350_c() {
        let tests = [
            TestCase(5, vec![3, 4, 1, 2, 5], (2, Some(vec![(1, 3), (2, 4)]))),
            TestCase(4, vec![1, 2, 3, 4], (0, None)),
            TestCase(3, vec![3, 1, 2], (2, Some(vec![(1, 2), (2, 3)]))),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
