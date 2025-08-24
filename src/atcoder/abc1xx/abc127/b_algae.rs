// https://atcoder.jp/contests/abc127/tasks/abc127_b

fn run(r: usize, d: usize, x: usize) -> Vec<usize> {
    (0..10)
        .scan(x, |state, _| {
            *state = r * *state - d;
            Some(*state)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>);

    #[test]
    fn abc127_b() {
        let tests = [
            TestCase(2, 10, 20, vec![30, 50, 90, 170, 330, 650, 1290, 2570, 5130, 10250]),
            TestCase(4, 40, 60, vec![200, 760, 3000, 11960, 47800, 191160, 764600, 3058360, 12233400, 48933560]),
        ];

        for TestCase(r, d, x, expected) in tests {
            assert_eq!(run(r, d, x), expected);
        }
    }
}
