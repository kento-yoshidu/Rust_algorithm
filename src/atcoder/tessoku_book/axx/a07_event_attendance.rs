// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_g

fn run(d: usize, _n: usize, lr: Vec<(usize, usize)>) -> Vec<isize> {
    let mut imos = vec![0; d+1];

    for (l, r) in lr {
        imos[l-1] += 1;
        imos[r] -= 1;
    }

    let mut ans = vec![0; d];

    ans[0] = imos[0];

    for i in 1..imos.len()-1 {
        ans[i] = ans[i - 1] + imos[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<isize>);

    #[test]
    fn tessoku_a07() {
        let tests = [
            TestCase(15, 3, vec![(2, 10), (3, 6), (9, 14)], vec![0, 1, 2, 2, 2, 2, 1, 1, 2, 2, 1, 1, 1, 1, 0]),
            TestCase(8, 5, vec![(2, 3), (3, 6), (5, 7), (3, 7), (1, 5)], vec![1, 2, 4, 3, 4, 3, 2, 0]),
        ];

        for TestCase(d, n, lr, expected) in tests {
            assert_eq!(run(d, n, lr), expected);
        }
    }
}
