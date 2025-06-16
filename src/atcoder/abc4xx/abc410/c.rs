// https://atcoder.jp/contests/abc410/tasks/abc410_c

fn run(n: usize, _q: usize, query: Vec<(usize, usize, Option<usize>)>) -> Vec<usize> {
    let mut ans = Vec::new();

    let mut vec: Vec<usize> = (1..=n).collect();

    let mut pos = 0;

    for (p, x, k) in query {
        match p {
            1 => {
                vec[(pos + x - 1) % n] = k.unwrap();
            },
            2 => {
                ans.push(vec[(pos + x - 1) % n]);
            },
            3 => {
                pos += x;
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, Option<usize>)>, Vec<usize>);

    #[test]
    fn abc410_c() {
        let tests = [
            TestCase(5, 5, vec![(2, 3, None), (1, 2, Some(1000000)), (3, 4, None), (2, 2, None), (2, 3, None)], vec![3, 1, 1000000]),
            TestCase(1000000, 2, vec![(1, 1000000, Some(999999)), (3, 1000000000, None)], vec![]),
        ];

        for TestCase(n, q, query, expected) in tests {
            assert_eq!(run(n, q, query), expected);
        }
    }
}
