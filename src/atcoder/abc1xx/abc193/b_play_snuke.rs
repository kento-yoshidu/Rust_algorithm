// https://atcoder.jp/contests/abc193/tasks/abc193_b

fn run(_n: usize, a: Vec<(isize, isize, isize)>) -> isize {
    a.into_iter()
        .filter(|t| { t.0 < t.2 })
        .map(|t| { t.1 })
        .min()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize, isize)>, isize);

    #[test]
    fn abc193_b() {
        let tests = [
            TestCase(3, vec![(3, 9, 5), (4, 8, 5), (5, 7, 5)], 8),
            TestCase(3, vec![(5, 9, 5), (6, 8, 5), (7, 7, 5)], -1),
            TestCase(10, vec![(158260522, 877914575, 602436426), (24979445, 861648772, 623690081), (433933447, 476190629, 262703497), (211047202, 971407775, 628894325), (731963982, 822804784, 450968417), (430302156, 982631932, 161735902), (880895728, 923078537, 707723857), (189330739, 910286918, 802329211), (404539679, 303238506, 317063340), (492686568, 773361868, 125660016)], 861648772),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
