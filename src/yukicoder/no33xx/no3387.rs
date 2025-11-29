// https://yukicoder.me/problems/no/3387

pub fn run(n: usize, a: Vec<usize>) -> &'static str {
    let m = a[0] + a[n-1];

    for i in 0..(n+1)/2 {
        if a[i] + a[n-1-i] != m {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn yuki_no3387() {
        let tests = [
            TestCase(5, vec![2, 3, 5, 7, 8], "Yes"),
            TestCase(4, vec![2, 3, 5, 7], "No"),
            TestCase(1, vec![100], "Yes"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }

}