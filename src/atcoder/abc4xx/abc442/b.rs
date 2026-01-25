// https://atcoder.jp/contests/abc442/tasks/abc442_b

fn run(_q: usize, a: Vec<usize>) -> Vec<&'static str> {
    a.into_iter()
        .scan((0usize, false), |(v, b), x| {
            match x {
                1 => *v += 1,
                2 => *v = (*v).saturating_sub(1),
                3 => *b = !*b,
                _ => unreachable!(),
            }

            Some(if *b && *v >= 3 { "Yes" } else { "No" })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::hint::assert_unchecked;

    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<&'static str>);

    #[test]
    fn abc442_b() {
        let tests = [
            TestCase(10, vec![2, 1, 3, 1, 3, 1, 1, 3, 2, 2], vec!["No", "No", "No", "No", "No", "No", "No", "Yes", "Yes", "No"]),
        ];

        for TestCase(q, a, expected) in tests {
            assert_eq!(run(q, a), expected);
        }
    }
}
