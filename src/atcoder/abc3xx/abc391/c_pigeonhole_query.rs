// https://atcoder.jp/contests/abc391/tasks/abc391_c

fn run(n: usize, _q: usize, query: Vec<(usize, Option<(usize, usize)>)>) -> Vec<usize> {
    let mut counts = vec![1; n];
    let mut pos: Vec<usize>= (0..n).collect();

    let mut count = 0;

    let mut ans = Vec::new();

    for (n, ph) in query {
        match n {
            1 => {
                let (p, h) = ph.unwrap();

                if counts[pos[p-1]] == 2 {
                    count -= 1;
                }
                counts[pos[p-1]] -= 1;

                pos[p-1] = h - 1;

                if counts[pos[p-1]] == 1 {
                    count += 1;
                }
                counts[pos[p-1]] += 1;
            },
            2 => {
                ans.push(count);
            }
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, Option<(usize, usize)>)>, Vec<usize>);

    #[test]
    fn abc391_c() {
        let tests = [
            TestCase(4, 7, vec![(2, None), (1, Some((1, 2))), (2, None), (1, Some((3, 2))), (2, None), (1, Some((3, 4))), (2, None)], vec![0, 1, 1, 2]),
            TestCase(5, 10, vec![(2, None), (1, Some((4, 3))), (1, Some((4, 5))), (2, None), (1, Some((3, 1))), (2, None), (1, Some((2, 3))), (1, Some((2, 5))), (1, Some((1, 3))), (2, None)], vec![0, 1, 2, 1]),
        ];

        for TestCase(n, q, query, expected) in tests {
            assert_eq!(run(n, q, query), expected);
        }
    }
}
