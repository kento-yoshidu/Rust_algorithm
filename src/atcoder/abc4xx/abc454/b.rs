// https://atcoder.jp/contests/abc454/tasks/abc454_b

fn run(_n: usize, m: usize, f: Vec<usize>) -> (&'static str, &'static str) {
    let mut a = vec![0; m];

    for i in f {
        a[i-1] += 1;
    }

    let x =
        if a.iter().all(|c| *c < 2) {
            "Yes"
        } else {
            "No"
        };

    let y =
        if a.iter().all(|c| *c >= 1) {
            "Yes"
        } else {
            "No"
        };

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, (&'static str, &'static str));

    #[test]
    fn abc454_b() {
        let tests = [
            TestCase(3, 4, vec![1, 2, 4], ("Yes", "No")),
            TestCase(4, 2, vec![1, 2, 1, 2], ("No", "Yes")),
            TestCase(4, 4, vec![1, 3, 2, 1], ("No", "No")),
            TestCase(5, 5, vec![1, 3, 4, 2, 5], ("Yes", "Yes")),
        ];

        for TestCase(n, m, f, expected) in tests {
            assert_eq!(run(n, m, f), expected);
        }
    }
}
