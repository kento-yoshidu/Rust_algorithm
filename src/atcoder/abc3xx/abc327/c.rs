// https://atcoder.jp/contests/abc327/tasks/abc327_c

use itertools::Itertools;

fn run(a: Vec<Vec<usize>>) -> &'static str {
    // 横方向に重複がないか
    for v in a.iter() {
        if !v.iter().all_unique() {
            return "No";
        }
    }

    // 縦方向に重複がないか
    for i in 0..9 {
        let mut col = Vec::new();

        for j in 0..9 {
            col.push(a[j][i]);
        }

        if !col.iter().all_unique() {
            return "No";
        }
    }

    // 3x3に重複がないか
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            let mut square = Vec::new();

            for ti in 0..3 {
                for tj in 0..3 {
                    square.push(a[i + ti][j + tj]);
                }
            }

            if !square.iter().all_unique() {
                return "No";
            }
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<Vec<usize>>, &'static str);

    #[test]
    fn abc327_c() {
        let tests = [
            TestCase(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![2, 3, 4, 5, 6, 7, 8, 9, 1], vec![5, 6, 7, 8, 9, 1, 2, 3, 4], vec![8, 9, 1, 2, 3, 4, 5, 6, 7], vec![3, 4, 5, 6, 7, 8, 9, 1, 2], vec![6, 7, 8, 9, 1, 2, 3, 4, 5], vec![9, 1, 2, 3, 4, 5, 6, 7, 8]], "Yes"),
            TestCase(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![2, 3, 4, 5, 6, 7, 8, 9, 1], vec![3, 4, 5, 6, 7, 8, 9, 1, 2], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![5, 6, 7, 8, 9, 1, 2, 3, 4], vec![6, 7, 8, 9, 1, 2, 3, 4, 5], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![8, 9, 1, 2, 3, 4, 5, 6, 7], vec![9, 1, 2, 3, 4, 5, 6, 7, 8]], "No"),
            TestCase(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8, 9, 1, 2, 3], vec![7, 8, 9, 1, 2, 3, 4, 5, 6]], "No"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
