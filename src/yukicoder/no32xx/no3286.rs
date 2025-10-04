// https://yukicoder.me/problems/no/3286

fn run(n: usize, a: Vec<usize>) -> &'static str {
    for i in 0..n-1 {
        if a[i] == a[i+1] {
            return "Yes";
        }
    }

    for i in 0..n-2 {
        if a[i] == a[i+2] {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn yuki_3286() {
        let tests = [
            TestCase(4, vec![1, 3, 2, 3], "Yes"),
            TestCase(5, vec![1, 1, 1, 1, 1], "Yes"),
            TestCase(5, vec![1, 2, 3, 4, 1], "No"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
