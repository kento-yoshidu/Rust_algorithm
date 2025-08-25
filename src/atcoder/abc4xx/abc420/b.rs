// https://atcoder.jp/contests/abc420/tasks/abc420_b

use itertools::Itertools;

fn run(n: usize, m: usize, s: Vec<&str>) -> Vec<usize> {
    let chars: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut point = vec![0; n];

    for i in 0..m {
        let arr = (0..n).map(|j| chars[j][i]);

        if arr.clone().all(|c| c == '0') {
            point.iter_mut().for_each(|p| *p += 1);
            continue;
        }

        let zero = arr.clone().filter(|c| *c == '0').count();
        let one = n - zero;

        let target =
            if zero < one {
                '0'
            } else {
                '1'
            };

        for (j, c) in arr.enumerate() {
            if c == target {
                point[j] += 1;
            }
        }
    }

    point.into_iter()
        .enumerate()
        .max_set_by_key(|&(_, val)| val)
        .into_iter()
        .map(|(i, _)| i+1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<usize>);

    #[test]
    fn abc420_b() {
        let tests = [
            TestCase(3, 5, vec!["11100", "10101", "01110"], vec![2, 3]),
            TestCase(5, 4, vec!["0000", "0000", "0000", "0000", "0000"], vec![1, 2, 3, 4, 5]),
            TestCase(7, 8, vec!["11010011", "01000000", "01111100", "10111000", "10011110", "10100101", "10010110"], vec![1, 2, 3]),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, s), expected);
        }
    }
}
