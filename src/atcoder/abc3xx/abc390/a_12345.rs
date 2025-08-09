// https://atcoder.jp/contests/abc390/tasks/abc390_a

fn run(a: Vec<usize>) -> &'static str {
    let mut vec = a.clone();

    let mut count = 0;

    for i in 0..4 {
        if i+1 != vec[i] {
            if count == 1 {
                return "No";
            } else {
                vec.swap(i, i+1);
                count += 1;
            }
        }
    }

    if count == 0 {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![1, 2, 4, 3, 5], "Yes"),
            TestCase(vec![5, 3, 2, 4, 1], "No"),
            TestCase(vec![1, 2, 3, 4, 5], "No"),
            TestCase(vec![2, 1, 3, 4, 5], "Yes"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
