// https://atcoder.jp/contests/awc0003/tasks/awc0003_b

fn run(_n: usize, lr: Vec<(char, char)>) -> usize {
    lr.windows(2)
        .filter(|arr| arr[0].1 == arr[1].0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(char, char)>, usize);

    #[test]
    fn awc0003_b() {
        let tests = [
            TestCase(4, vec![('N', 'S'), ('S', 'N'), ('N', 'N'), ('S', 'S')], 2),
            TestCase(2, vec![('N', 'N'), ('N', 'S')], 1),
            TestCase(10, vec![('S', 'S'), ('S', 'N'), ('N', 'S'), ('S', 'S'), ('S', 'N'), ('N', 'N'), ('N', 'S'), ('S', 'S'), ('S', 'N'), ('N', 'N')], 9),
        ];

        for TestCase(n, lr, expected) in tests {
            assert_eq!(run(n, lr), expected);
        }
    }
}
