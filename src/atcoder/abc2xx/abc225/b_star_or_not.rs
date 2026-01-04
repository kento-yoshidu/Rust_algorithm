// https://atcoder.jp/contests/abc225/tasks/abc225_b

fn run(n: usize, ab: Vec<(usize, usize)>) -> &'static str {
    let count = ab.iter()
        .fold(vec![0; n], |mut state, (a, b)| {
            state[*a-1] += 1;
            state[*b-1] += 1;
            state
        });

    if count.iter()
        .any(|num| {
            *num == n - 1
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc225_b() {
        let tests = [
            TestCase(5, vec![(1, 4), (2, 4), (3, 4), (4, 5)], "Yes"),
            TestCase(4, vec![(2, 4), (1, 4), (2, 3)], "No"),
            TestCase(10, vec![(9, 10), (3, 10), (4, 10), (8, 10), (1, 10), (2, 10), (7, 10), (6, 10), (5, 10)], "Yes"),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
