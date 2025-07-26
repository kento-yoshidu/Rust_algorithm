// https://atcoder.jp/contests/abc122/tasks/abc122_c

fn run(n: usize, _q: usize, s: &str, lr: Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans = Vec::new();

    let mut cum_sum = vec![0; n];

    let chars = s.chars().collect::<Vec<char>>();

    for (i, lr) in chars.windows(2).enumerate() {
        if lr[0] == 'A' && lr[1] == 'C' {
            cum_sum[i+1] = cum_sum[i]+1;
        } else {
            cum_sum[i+1] = cum_sum[i];
        }
    }

    for (l, r) in lr.iter() {
        ans.push(cum_sum[r-1] - cum_sum[l-1]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc122_c() {
        let tests = [
            TestCase(8, 3, "ACACTACG", vec![(3, 7), (2, 3), (1, 8)], vec![2, 0, 3]),
        ];

        for TestCase(n, q, s, lr, expected) in tests {
            assert_eq!(run(n, q, s, lr), expected);
        }
    }
}
