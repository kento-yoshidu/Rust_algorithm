// https://atcoder.jp/contests/abc449/tasks/abc449_b

fn run(h: usize, w: usize, _q: usize, query: Vec<(usize, usize)>) -> Vec<usize> {
    let mut h = h;
    let mut w = w;

    let mut ans = Vec::new();

    for (r, c) in query {
        match r {
            1 => {
                ans.push(w * c);
                h -= c;
            },
            2 => {
                ans.push(h * c);
                w -= c;
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc449_b() {
        let tests = [
            TestCase(7, 9, 5, vec![ (2, 4), (1, 3), (2, 1), (2, 1), (1, 3)], vec![28, 15, 4, 4, 9]),
        ];

        for TestCase(h, w, q, query, expected) in tests {
            assert_eq!(run(h, w, q, query), expected);
        }
    }
}
