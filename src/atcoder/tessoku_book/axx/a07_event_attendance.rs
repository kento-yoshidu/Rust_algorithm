// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_g

fn run(d: usize, n: usize, vec: Vec<(isize, isize)>) -> Vec<isize> {
    let mut tmp = vec![0; d];

    for i in 0..n {
        tmp[((vec[i].0) -1) as usize] += 1;
        tmp[((vec[i].1)) as usize] -= 1;
    }

    let mut ans = vec![0; d];


    ans[0] = tmp[0];

    for i in 1..tmp.len() {
        ans[i] = ans[i - 1] + tmp[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(isize, isize)>, Vec<isize>);

    #[test]
    fn tessoku_a07() {
        let tests = [
            TestCase(15, 3, vec![(2, 10), (3, 6), (9, 14)], vec![0, 1, 2, 2, 2, 2, 1, 1, 2, 2, 1, 1, 1, 1, 0]),
            TestCase(8, 5, vec![(2, 3), (3, 6), (5, 7), (3, 7), (1, 5)], vec![1, 2, 4, 3, 4, 3, 2, 0]),
        ];

        for TestCase(d, n, vec, expected) in tests {
            assert_eq!(run(d, n, vec), expected);
        }
    }
}
