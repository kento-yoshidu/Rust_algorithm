// https://atcoder.jp/contests/abc382/tasks/abc382_c

use itertools::Itertools;

fn run(_n: usize, m: usize, a: Vec<usize>, b: Vec<isize>) -> Vec<isize> {
    let vec: Vec<(usize, isize)> = b.into_iter()
        .enumerate()
        .sorted_by(|a, b| b.1.cmp(&(a.1)))
        .collect();

    println!("{:?}", vec);

    let mut ans = vec![-1; m];

    let mut idx = 0;

    for (i, x) in a.iter().enumerate() {
        while idx < m {
            if vec[idx].1 < *x as isize {
                break;
            }

            ans[vec[idx].0] = i as isize+1;
            idx += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<isize>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec![3, 8, 2], vec![5, 2, 1], vec![1, 3, -1]),
            TestCase(3, 3, vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]),
            TestCase(10, 5, vec![60, 83, 76, 45, 70, 91, 37, 58, 94, 22], vec![70, 39, 52, 33, 18], vec![1, 7, 4, 10, -1]),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
