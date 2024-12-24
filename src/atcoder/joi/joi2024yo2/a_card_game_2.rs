// https://atcoder.jp/contests/joi2024yo2/tasks/joi2024_yo2_a

fn run(_n: usize, a: Vec<usize>) -> &'static str {
    let mut vec = vec![false; 200_000];

    for i in a {
        vec[i-1] = true;
    }

    for i in 0..200_000-6 {
        if vec[i] == true && vec[i+3] == true && vec[i+6] == true {
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
    fn test() {
        let tests = [
            TestCase(3, vec![2, 5, 8], "Yes"),
            TestCase(4, vec![1, 4, 6, 4], "No"),
            TestCase(8, vec![9, 8, 11, 1, 1, 6, 10, 4], "No"),
            TestCase(20, vec![2, 15, 4, 30, 6, 8, 11, 27, 14, 3, 16, 26, 19, 2, 23, 21, 18, 13, 28, 6], "Yes"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
