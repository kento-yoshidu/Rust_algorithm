// https://atcoder.jp/contests/abc392/tasks/abc392_a

use itertools::Itertools;

fn run(a: [usize; 3]) -> &'static str {
    let a: Vec<usize> = a.into_iter().sorted().collect();

    if a[0] * a[1] == a[2] {
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
    fn test() {
        let tests = [
            TestCase([3, 15, 5], "Yes"),
            TestCase([5, 3, 2], "No"),
            TestCase([3, 3, 9], "Yes"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
