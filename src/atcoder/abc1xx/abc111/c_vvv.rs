// https://atcoder.jp/contests/abc111/tasks/arc103_a

use std::collections::HashMap;
use std::cmp::max;

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut odd = HashMap::new();
    let mut even = HashMap::new();

    for (i, num) in a.iter().enumerate() {
        if i % 2 == 0 {
            *odd.entry(num).or_insert(0) += 1;
        } else {
            *even.entry(num).or_insert(0) += 1;
        }
    }

    let mut vec_odd: Vec<(usize, usize)> = odd.into_iter().map(|(k, v)| (*k, v)).collect();
    let mut vec_even: Vec<(usize, usize)> = even.into_iter().map(|(k, v)| (*k, v)).collect();

    vec_odd.sort_by(|a, b| b.1.cmp(&a.1));
    vec_even.sort_by(|a, b| b.1.cmp(&a.1));

    // 最頻の数値が違う場合
    if vec_odd[0].0 != vec_even[0].0 {
        n - (vec_odd[0].1 + vec_even[0].1)
    } else {
        // 全ての同じ数値の場合、2番目に0を設定しておく
        let odd_second = if vec_odd.len() > 1 {
            vec_odd[1].1
        } else {
            0
        };

        let even_second = if vec_even.len() > 1 {
            vec_even[1].1
        } else {
            0
        };

        n - max(vec_odd[0].1 + even_second, odd_second + vec_even[0].1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![3, 1, 3, 2], 1),
            TestCase(6, vec![105, 119, 105, 119, 105, 119], 0),
            TestCase(4, vec![1, 1, 1, 1], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
