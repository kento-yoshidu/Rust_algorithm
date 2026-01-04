// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ce

fn run(_n: usize, a: Vec<usize>, _q: usize, lr: Vec<(usize, usize)>) -> Vec<&'static str> {
    let mut cum_sum: Vec<isize> = Vec::from([0]);

    for (i, c) in a.iter().enumerate() {
        if *c == 1 {
            cum_sum.push(cum_sum[i] + 1);
        } else {
            cum_sum.push(cum_sum[i] - 1);
        }
    }

    lr.into_iter()
        .map(|(l, r)| {
            let res = cum_sum[r] - cum_sum[l-1];

            if res >= 1 {
                "win"
            } else if res == 0 {
                "draw"
            } else {
                "lose"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<(usize, usize)>, Vec<&'static str>);

    #[test]
    fn tessoku_b06() {
        let tests = [
            TestCase(7, vec![0, 1, 1, 0, 1, 0, 0], 3, vec![(2, 5), (2, 7), (5, 7)], vec!["win", "draw", "lose"]),
        ];

        for TestCase(n, a, q, lr, expected) in tests {
            assert_eq!(run(n, a, q, lr), expected);
        }
    }
}
