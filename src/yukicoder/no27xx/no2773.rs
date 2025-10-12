// https://yukicoder.me/problems/no/2773

use itertools::Itertools;

fn run(_n: usize, s: &str) -> (usize, Option<Vec<usize>>) {
    let ans: Vec<usize> = s.chars()
        .tuple_windows::<(_, _)>()
        .enumerate()
        .filter(|(_, a)| a.0 == 'x' && a.1 == 'o')
        .map(|(i, _)| i+2)
        .collect();

    if ans.len() == 0 {
        (0, None)
    } else {
        (ans.len(), Some(ans))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, (usize, Option<Vec<usize>>));

    #[test]
    fn yuki_2773() {
        let tests = [
            TestCase(8, "xxoooxxo", (2, Some(vec![3, 8]))),
            TestCase(1, "x", (0, None)),
            TestCase(19, "xooxoooxxxxoooxooox", (4, Some(vec![2, 5, 12, 16]))),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
