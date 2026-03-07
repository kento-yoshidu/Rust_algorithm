// https://atcoder.jp/contests/abc261/tasks/abc261_b

fn run(n: usize, a: Vec<&str>) -> &'static str {
    let vec: Vec<Vec<char>> = a.iter().map(|a| a.chars().collect()).collect();

    for i in 0..n {
        for j in 0..n {
            if i == j { continue };

            if vec[i][j] == 'W' && vec[j][i] != 'L' {
                return "incorrect";
            } else if vec[i][j] == 'L' && vec[j][i] != 'W' {
                return  "incorrect"
            } else if vec[i][j] == 'D' && vec[j][i] != 'D' {
                return "incorrect"
            }
        }
    }

    "correct"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc261_b() {
        let tests = [
            TestCase(4, vec!["-WWW", "L-DD", "LD-W", "LDW-"], "incorrect"),
            TestCase(2, vec!["-D", "D-"], "correct"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
