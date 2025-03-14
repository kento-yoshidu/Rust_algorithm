// https://atcoder.jp/contests/abc038/tasks/abc038_b

fn run(a: Vec<usize>, b: Vec<usize>) -> &'static str {
    if a.iter().any(|a_num| {
        b.iter().any(|b_num| {
            a_num == b_num
        })
    }) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![1080, 1920], vec![1080, 1920], "Yes"),
            TestCase(vec![1080, 1920], vec![1920, 1080], "Yes"),
            TestCase(vec![334, 668], vec![669, 1002], "No"),
            TestCase(vec![100, 200], vec![300, 150], "No"),
            TestCase(vec![120, 120], vec![240, 240], "No"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
