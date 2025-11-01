// https://atcoder.jp/contests/abc136/tasks/abc136_chttps://atcoder.jp/contests/abc136/tasks/abc136_c

fn run(n: usize, h: Vec<isize>) -> &'static str {
    let mut vec: Vec<isize> = h.clone().into_iter().rev().collect();

    for i in 0..n-1 {
        if vec[i] + 1 == vec[i+1] {
            vec[i+1] -= 1;
        } else if vec[i+1] - vec[i] >= 2 {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, &'static str);

    #[test]
    fn abc136_c() {
        let tests = [
            TestCase(5, vec![1, 2, 1, 1, 3], "Yes"),
            TestCase(4, vec![1, 3, 2, 1], "No"),
            TestCase(5, vec![1, 2, 3, 4, 5], "Yes"),
            TestCase(1, vec![1000000000], "Yes"),
            TestCase(5, vec![1, 1, 3, 1, 1], "No"),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
