// https://atcoder.jp/contests/abc431/tasks/abc431_c

use itertools::Itertools;

fn run(_n: usize, _m: usize, k: usize, h: Vec<usize>, b: Vec<usize>) -> &'static str {
    let head: Vec<usize> = h.into_iter().sorted().take(k).collect();
    let body: Vec<usize> = b.into_iter().sorted().rev().take(k).rev().collect();

    if head.into_iter()
        .zip(body)
        .all(|(h, b)| h <= b ) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn abc431_c() {
        let tests = [
            TestCase(6, 6, 3, vec![2, 7, 1, 8, 2, 8], vec![1, 8, 2, 8, 4, 5], "Yes"),
            TestCase(1, 1, 1, vec![43], vec![1], "No"),
            TestCase(1, 1, 1, vec![100], vec![100], "Yes"),
            TestCase(12, 15, 12, vec![748, 169, 586, 329, 972, 529, 432, 519, 408, 587, 138, 249], vec![656, 114, 632, 299, 984, 755, 404, 772, 155, 506, 832, 854, 353, 465, 387], "Yes"),
        ];

        for TestCase(n, m, k, h, b, expected) in tests {
            assert_eq!(run(n, m, k, h, b), expected);
        }
    }
}
