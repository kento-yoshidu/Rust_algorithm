// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/5/ITP1_5_A

fn run(hw: Vec<(usize, usize)>) -> Vec<Vec<String>> {
    hw.into_iter()
        .filter_map(|(h, w)| {
            match h {
                0 => None,
                _ => {
                    Some(vec!["#".repeat(w); h])
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(usize, usize)>, Vec<Vec<&'static str>>);

    #[test]
    fn itp1_5_a() {
        let tests = [
            TestCase(vec![(3, 4), (5, 6), (2, 2), (0, 0)], vec![vec!["####", "####", "####"], vec!["######", "######", "######", "######", "######"], vec!["##", "##"]]),
        ];

        for TestCase(hw, expected) in tests {
            assert_eq!(run(hw), expected);
        }
    }
}
