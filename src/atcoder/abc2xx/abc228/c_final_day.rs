// https://atcoder.jp/contests/abc228/tasks/abc228_c

fn run(n: usize, k: usize, p: Vec<Vec<usize>>) -> Vec<&'static str> {
    let vec: Vec<usize> = p.into_iter()
        .map(|t| t.iter().sum())
        .collect();

    let mut vec_clone = vec.clone();
    vec_clone.sort();

    vec.into_iter()
        .map(|num| {
            if num + 300 > vec_clone[n-k-1] {
                "Yes"
            } else {
                "No"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, Vec<&'static str>);

    #[test]
    fn abc228_c() {
        let tests = [
            TestCase(3, 1, vec![vec![178, 205, 132], vec![112, 220, 96], vec![36, 64, 20]], vec!["Yes", "Yes", "No"]),
            TestCase(2, 1, vec![vec![300, 300, 300], vec![200, 200, 200]], vec!["Yes", "Yes"]),
            TestCase(4, 2, vec![vec![127, 235, 78], vec![192, 134, 298], vec![28, 56, 42], vec![96, 120, 250]], vec!["Yes", "Yes", "No", "Yes"]),
        ];

        for TestCase(n, k, p, expected) in tests {
            assert_eq!(run(n, k, p), expected);
        }
    }
}
