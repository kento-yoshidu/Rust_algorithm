// https://atcoder.jp/contests/abc307/tasks/abc307_a

use itertools::Itertools;

fn run(n: usize, a: &Vec<usize>) -> Vec<usize> {
    let mut ans =Vec::<usize>::new();

    for _i in 0..n {
        ans.push(0)
    }

    for (i, v) in a.into_iter().enumerate() {
        ans[i/7] += v;
    }

    ans
}

fn run2(_n: usize, a: &Vec<usize>) -> Vec<usize> {
    a
        .chunks(7)
        .map(|vec| vec.iter().sum())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc307_a() {
        let tests = [
            TestCase(2, vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 2000, 3000, 4000, 5000, 6000, 7000, 8000], vec![28000, 35000]),
            TestCase(3, vec![14159, 26535, 89793, 23846, 26433, 83279, 50288, 41971, 69399, 37510, 58209, 74944, 59230, 78164, 6286, 20899, 86280, 34825, 34211, 70679, 82148], vec![314333, 419427, 335328]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
