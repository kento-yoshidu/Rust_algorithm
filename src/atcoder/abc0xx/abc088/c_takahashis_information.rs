// https://atcoder.jp/contests/abc088/tasks/abc088_c

pub fn run(c: Vec<Vec<isize>>) -> &'static str {
    let s1 = c[0][0] + c[1][1] + c[2][2];
    let s2 = c[0][1] + c[1][2] + c[2][0];
    let s3 = c[0][2] + c[1][0] + c[2][1];

    if s1 == s2 && s2 == s3 && s3 == s1 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<Vec<isize>>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![vec![1, 0, 1], vec![2, 1, 2], vec![1, 0, 1]], "Yes"),
            TestCase(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]], "No"),
            TestCase(vec![vec![0, 8, 8], vec![0, 8, 8], vec![0, 8, 8]], "Yes"),
            TestCase(vec![vec![1, 8, 6], vec![2, 9, 7], vec![0, 7, 7]], "No"),
        ];

        for TestCase(c, expected) in tests {
            assert_eq!(run(c), expected);
        }
    }
}
