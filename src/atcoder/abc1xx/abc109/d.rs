// https://atcoder.jp/contests/abc109/tasks/abc109_d

fn run(h: usize, w: usize, a: Vec<Vec<usize>>) -> Vec<(usize, usize, usize, usize)> {
    let mut vec = a.clone();

    let mut ans = Vec::new();

    for i in 0..h {
        for j in 0..w-1 {
            if vec[i][j] % 2 != 0 {
                vec[i][j+1] += 1;

                ans.push((i+1, j+1, i+1, j+2));
            }
        }
    }

    for i in 0..h-1 {
        if vec[i][w-1] % 2 != 0 {
            vec[i+1][w-1] += 1;

            ans.push((i+1, w, i+2, w));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, Vec<(usize, usize, usize, usize)>);

    #[test]
    fn abc109_d() {
        let tests = [
            TestCase(2, 3, vec![vec![1, 2, 3], vec![0, 1, 1]], vec![(1, 1, 1, 2), (1, 2, 1, 3), (2, 2, 2, 3)]),
            TestCase(3, 2, vec![vec![1, 0], vec![2, 1], vec![1, 0]], vec![(1, 1, 1, 2), (3, 1, 3, 2), (1, 2, 2, 2)]),
            TestCase(1, 5, vec![vec![9, 9, 9, 9, 9]], vec![(1, 1, 1, 2), (1, 3, 1, 4)]),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, a), expected);
        }
    }
}
