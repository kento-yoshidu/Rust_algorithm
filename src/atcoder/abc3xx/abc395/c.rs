// https://atcoder.jp/contests/abc395/tasks/abc395_c

use std::collections::HashMap;

fn run(_n: usize, a: &Vec<usize>) -> isize {
    let mut index =  vec![vec![]; 1000000+1];

    for (i, num) in a.into_iter().enumerate() {
        index[num-1].push(i);
    }

    index.into_iter()
        .filter(|arr| arr.len() > 1)
        .map(|arr| {
            (arr[1] - arr[0] + 1) as isize
        })
        .min()
        .unwrap_or(-1)
}

fn run2(_n: usize, a: &Vec<usize>) -> isize {
    let mut map = HashMap::new();

    for (i, a) in a.into_iter().enumerate() {
        map.entry(a).or_insert_with(Vec::new).push(i);
    }

    map.into_iter()
        .filter(|(_, v)| {
            v.len() > 1
        })
        .map(|(_, v)| {
            v.windows(2).map(|arr| (arr[1] - arr[0] + 1) as isize).min().unwrap()
        })
        .min()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![3, 9, 5, 3, 1], 4),
            TestCase(4, vec![2, 5, 3, 1], -1),
            TestCase(10, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
