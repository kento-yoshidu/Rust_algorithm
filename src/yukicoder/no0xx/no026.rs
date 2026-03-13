// https://yukicoder.me/problems/no/26

fn run(n: usize, _m: usize, pq: Vec<(usize, usize)>) -> usize {
    let mut pos = n;

    for (p, q) in pq {
        if pos == p {
            pos = q;
        } else if pos == q {
            pos = p;
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn yuki_026() {
        let tests = [
            TestCase(1, 1, vec![(1, 3)], 3),
            TestCase(1, 3, vec![(2, 3), (3, 2), (2, 3)], 1),
        ];

        for TestCase(n, m, pq, expected) in tests {
            assert_eq!(run(n, m, pq), expected);
        }
    }
}
