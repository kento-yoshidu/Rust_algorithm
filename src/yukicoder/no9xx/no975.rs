// https://yukicoder.me/problems/no/975

fn run(x: usize, _n: usize, _m: usize, a: &Vec<usize>, b: &Vec<usize>) -> &'static str {
    let fa = a.into_iter().find(|num| **num == x);
    let fb = b.into_iter().find(|num| **num == x);

    match (fa, fb) {
        (Some(_), Some(_)) => "MrMaxValu",
        (Some(_), None) => "MrMax",
        (None, Some(_)) => "MaxValu",
        (None, None) => "-1",
    }
}

fn run2(x: usize, _n: usize, _m: usize, a: &Vec<usize>, b: &Vec<usize>) -> &'static str {
    match (a.contains(&x), b.contains(&x)) {
        (true, true) => "MrMaxValu",
        (true, false) => "MrMax",
        (false, true) => "MaxValu",
        (false, false) => "-1",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn yuki_975() {
        let tests = [
            TestCase(114, 5, 8, vec![1, 3, 4, 9, 12], vec![2, 3, 11, 18, 114, 514, 364, 1919], "MaxValu"),
            TestCase(2525, 1, 1, vec![2525], vec![2525], "MrMaxValu"),
        ];

        for TestCase(x, n, m, a, b, expected) in tests {
            assert_eq!(run(x, n, m, &a, &b), expected);
            assert_eq!(run2(x, n, m, &a, &b), expected);
        }
    }
}
