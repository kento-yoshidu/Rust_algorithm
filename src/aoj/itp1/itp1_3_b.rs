// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/3/ITP1_3_B

fn run(case: Vec<usize>) -> Vec<String> {
    case.into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if x == 0 {
                None
            } else {
                Some(format!("Case {}: {x}", i + 1))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, Vec<&'static str>);

    #[test]
    fn itp1_3_b() {
        let tests = [
            TestCase(vec![3, 5, 11, 7, 8, 19, 0], vec!["Case 1: 3", "Case 2: 5", "Case 3: 11", "Case 4: 7", "Case 5: 8", "Case 6: 19"]),
        ];

        for TestCase(case, expected) in tests {
            assert_eq!(run(case), expected);
        }
    }
}
