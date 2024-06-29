// https://atcoder.jp/contests/abc356/tasks/abc356_b

pub fn run(n: usize, m: usize, a: Vec<usize>, x: Vec<Vec<usize>>) -> &'static str {
    let mut nutrients = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            nutrients[j] += x[i][j];
        }
    }

    if nutrients.into_iter()
        .enumerate()
        .all(|(i, num)| {
            num >= a[i]
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<Vec<usize>>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, vec![10, 20, 30], vec![vec![20, 0, 10], vec![0, 100, 100]], "Yes"),
            TestCase(2, 4, vec![10, 20, 30, 40], vec![vec![20, 0, 10, 30], vec![0, 100, 100, 0]], "No"),
        ];

        for TestCase(n, m, a, x, expected) in tests {
            assert_eq!(run(n, m, a, x), expected);
        }

    }
}
