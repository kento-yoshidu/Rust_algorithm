// https://atcoder.jp/contests/abc035/tasks/abc035_c

fn run(n: usize, _q: usize, lr: Vec<(usize, usize)>) -> String {
    let mut imos = vec![0; n];

    for (l, r) in lr {
        imos[l-1] += 1;

        if n != r {
            imos[r] -= 1;
        }
    }

    let mut acc = vec![imos[0]];

    for i in 1..n {
        acc.push(imos[i] + acc[i-1]);
    }

    acc.into_iter()
        .map(|n| {
            match n % 2 {
                0 => '0',
                1 => '1',
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc035_c() {
        let tests = [
            TestCase(5, 4, vec![(1, 4), (2, 5), (3, 3), (1, 5)], "01010"),
            TestCase(20, 8, vec![(1, 8), (4, 13), (8, 8), (3, 18), (5, 20), (19, 20), (2, 7), (4, 9)], "10110000011110000000"),
        ];

        for TestCase(n, q, lr, expected) in tests {
            assert_eq!(run(n, q, lr), expected);
        }
    }
}
