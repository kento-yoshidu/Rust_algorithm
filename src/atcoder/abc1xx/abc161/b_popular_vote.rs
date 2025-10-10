// https://atcoder.jp/contests/abc161/tasks/abc161_b

fn run(_n: usize, m: usize, a: Vec<usize>) -> &'static str {
    let total: usize = a.iter().sum();
    let border = (total as f64 / (4 * m) as f64).ceil() as usize;

    let count = a.iter()
        .filter(|num| {
            **num >= border
        })
        .count();

    if count >= m {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn abc161_b() {
        let tests = [
            TestCase(4, 1, vec![5, 4, 2, 1], "Yes"),
            TestCase(3, 2, vec![380, 19, 1], "No"),
            TestCase(12, 3, vec![4, 56, 78, 901, 2, 345, 67, 890, 123, 45, 6, 789], "Yes"),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
