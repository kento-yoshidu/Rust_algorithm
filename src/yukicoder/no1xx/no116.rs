// https://yukicoder.me/problems/no/116

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.windows(3)
        .filter(|arr| {
            let (x, y, z) = (arr[0], arr[1], arr[2]);

            x != y && y != z && x != z &&
            ((y > x && y > z) || (y < x && y < z))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn yuki_116() {
        let tests = [
            TestCase(5, vec![1, 3, 4, 1, 2], 2),
            TestCase(5, vec![1, 4, 2, 4, 1], 2),
            TestCase(5, vec![1, 4, 1, 5, 2], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
