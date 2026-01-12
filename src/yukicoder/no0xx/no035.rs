// https://yukicoder.me/problems/no/35

fn run(_n: usize, ts: Vec<(usize, &str)>) -> (usize, usize) {
    let mut ok = 0;
    let mut ng = 0;

    for (t, s) in ts {
        let l = s.len();
        let c = 12 * t / 1000;

        if l <= c {
            ok += l;
        } else {
            ok += c;
            ng += l - c;
        }
    }

    (ok, ng)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, &'static str)>, (usize, usize));

    #[test]
    fn yuki_035() {
        let tests = [
            TestCase(2, vec![(750, "yukicoder"), (749, "yukicoder")], (17, 1)),
            TestCase(4, vec![(83, "topcoder"), (417, "atcoder"), (29183, "yukicoder"), (1, "codeforces")], (14, 20)),
        ];

        for TestCase(n, ts, expected) in tests {
            assert_eq!(run(n, ts), expected);
        }
    }
}
