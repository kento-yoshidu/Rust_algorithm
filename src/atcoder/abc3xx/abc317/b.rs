// https://atcoder.jp/contests/abc317/tasks/abc317_b

use itertools::Itertools;

fn run(_n: usize, a: &Vec<usize>) -> usize {
    let mut a = a.clone();

    a.sort();

    a.windows(2)
        .find(|arr| {
            arr[1] - arr[0] > 1
        })
        .unwrap()[0] + 1
}

fn run2(_n: usize, a: &Vec<usize>) -> usize {
    a.into_iter()
        .sorted()
        .tuple_windows()
        .find_map(|(l, r)| {
            if r - l > 1 {
                Some(l + 1)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc317_b() {
        let tests = [
            TestCase(3, vec![2, 3, 5], 4),
            TestCase(8, vec![3, 1, 4, 5, 9, 2, 6, 8], 7),
            TestCase(16, vec![152, 153, 154, 147, 148, 149, 158, 159, 160, 155, 156, 157, 144, 145, 146, 150], 151),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
