// https://atcoder.jp/contests/abc309/tasks/abc309_b

fn run(n: usize, a: Vec<&str>) -> Vec<String> {
    let vec: Vec<Vec<char>> = a.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();

    let mut ans = vec![vec!['.'; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == 0 || j == 0 || i == n-1 || j == n-1 {
                if i == 0 && j < n-1 {
                    ans[i][j+1] = vec[i][j];
                }
                if i < n-1 && j == n-1 {
                    ans[i+1][j] = vec[i][j];
                }
                if i == n-1 && 0 < j {
                    ans[i][j-1] = vec[i][j];
                }
                if 0 < i && j == 0 {
                    ans[i-1][j] = vec[i][j];
                }
            } else {
                ans[i][j] = vec[i][j];
            }
        }
    }

    ans.into_iter()
        .map(|v| v.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec!["0101", "1101", "1111", "0000"], vec!["1010", "1101", "0111", "0001"]),
            TestCase(2, vec!["11", "11"], vec!["11", "11"]),
            TestCase(5, vec!["01010", "01001", "10110", "00110", "01010"], vec!["00101", "11000", "00111", "00110", "10100"]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
