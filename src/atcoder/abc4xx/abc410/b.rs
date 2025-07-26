// https://atcoder.jp/contests/abc410/tasks/abc410_b

fn run(n: usize, _q: usize, x: Vec<usize>) -> Vec<usize> {
    let mut vec = vec![0; n];

    let mut ans = Vec::new();

    for n in x {
        match n {
            0 => {
                let min = vec.iter().min().unwrap();

                let (i, _) = vec.iter().enumerate().find(|(_, n)| min == *n).unwrap();

                vec[i] += 1;
                ans.push(i+1);
            },
            _ => {
                vec[n-1] += 1;
                ans.push(n);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc410_b() {
        let tests = [
            TestCase(4, 5, vec![2, 0, 3, 0, 0], vec![2, 1, 3, 4, 1,]),
            TestCase(3, 7, vec![1, 1, 0, 0, 0, 0, 0], vec![1, 1, 2, 3, 2, 3, 1]),
            TestCase(6, 20, vec![4, 6, 0, 3, 4, 2, 6, 5, 2, 3, 0, 3, 2, 5, 0, 3, 5, 0, 2, 0], vec![4, 6, 1, 3, 4, 2, 6, 5, 2, 3, 1, 3, 2, 5, 1, 3, 5, 4, 2, 6]),
        ];

        for TestCase(n, q, x, expected) in tests {
            assert_eq!(run(n, q, x), expected);
        }
    }
}
