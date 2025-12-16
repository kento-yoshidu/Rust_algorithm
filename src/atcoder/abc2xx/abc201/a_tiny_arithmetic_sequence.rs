// https://atcoder.jp/contests/abc201/tasks/abc201_a

fn run(a: [usize; 3]) -> &'static str {
    let mut vec = a.clone();

    vec.sort();

    if vec[2] - vec[1] == vec[1] - vec[0] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 3], &'static str);

    #[test]
    fn abc201_a() {
        let tests = [
            TestCase([5, 1, 3], "Yes"),
            TestCase([1, 4, 3], "No"),
            TestCase([5, 5, 5], "Yes"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
