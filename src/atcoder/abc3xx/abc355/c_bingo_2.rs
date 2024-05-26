// https://atcoder.jp/contests/abc355/tasks/abc355_c

pub fn run(n: usize, _t: usize, a: Vec<usize>) -> isize {
    let mut vec = vec![vec![false; n]; n];

    for (i, num) in a.into_iter().enumerate() {
        let row = if num % n == 0 {
            num / n -1
        } else {
            num / n
        };

        let col = num - row * n - 1;

        vec[row][col] = true;

        println!("{:?}", vec);

        // 行チェック
        if vec[row].iter().all(|b| {
            *b == true
        }) {
            return (i+1) as isize;
        }

        if (0..n).all(|j| {
            vec[j][col] == true
        }) {
            return (i+1) as isize;
        }

        // to右下
        if (0..n).all(|j| {
            vec[j][j] == true
        }) {
            return (i+1) as isize;
        }

        // to左下
        if (0..n).rev().all(|j| {
            let k = n - 1 - j;
            vec[j][k] == true
        }) {
            return (i+1) as isize;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, vec![5, 1, 8, 9, 7], 4),
            TestCase(3, 5, vec![4, 2, 9, 7, 5], -1),
            TestCase(4, 12, vec![13, 9, 6, 5, 2, 7, 16, 14, 8, 3, 10, 11], 9),
            TestCase(3, 3, vec![1, 5, 9], 3),
            TestCase(3, 3, vec![3, 5, 7], 3),
        ];

        for TestCase(n, t, a, expected) in tests {
            assert_eq!(run(n, t, a), expected);
        }
    }
}
