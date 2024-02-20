// https://atcoder.jp/contests/abc340/tasks/abc340_b

pub fn run(_q: usize, query: Vec<(usize, usize)>) -> Vec<usize> {
    let mut vec = Vec::new();
    let mut ans = Vec::new();

    for (a, b) in query {
        match a {
            1 => vec.push(b),
            2 => ans.push(vec[vec.len()-b]),
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![(1, 20), (1, 30), (2, 1), (1, 40), (2, 3)], vec![30, 20]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
