// https://atcoder.jp/contests/abc298/tasks/abc298_b

fn rotate(n: usize, a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; n]; n];

    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[n - 1 - j][i];
        }
    }

    result
}

pub fn run(n: usize, a: Vec<Vec<char>>, b: Vec<Vec<char>>) -> &'static str {
    let mut tmp = a.clone();

    for _ in 0..n {
        let mut flag = true;

        for i in 0..n {
            for j in 0..n {
                if tmp[i][j] == '1' && b[i][j] == '0' {
                    flag = false
                }
            }
        }

        if flag == true {
            return "Yes"
        }

        tmp = rotate(n, tmp);
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<char>>, Vec<Vec<char>>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![vec!['0', '1', '1'], vec!['1', '0', '0'], vec!['0', '1', '0']], vec![vec!['1', '1', '0'], vec!['0', '0', '1'], vec!['1', '1', '1']], "Yes"),
            TestCase(2, vec![vec!['0', '0'], vec!['0', '0']], vec![vec!['1', '1'], vec!['1', '1']], "Yes"),
            TestCase(5, vec![vec!['0', '0', '1', '1', '0'], vec!['1', '0', '0', '1', '0'], vec!['0', '0', '1', '0', '1'], vec!['0', '1', '0', '1', '0'], vec!['0', '1', '0', '0', '1']], vec![vec!['1', '1', '0', '0', '1'], vec!['0', '1', '1', '1', '0'], vec!['0', '0', '1', '1', '1'] ,vec!['1', '0', '1', '0', '1'], vec!['1', '1', '0', '1', '0']], "No"),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
