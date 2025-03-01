// https://atcoder.jp/contests/abc379/tasks/abc379_d

fn run(_q: usize, query: Vec<(usize, Option<usize>)>) -> Vec<usize> {
    let mut vec = Vec::new();

    let mut base: isize = 0;

    let mut ans: Vec<usize> = Vec::new();

    for (a, b) in query {
        match a {
            1 => {
                vec.push(-base);
            },
            2 => {
                base += b.unwrap() as isize;
            },
            3 => {
                let tmp = b.unwrap() as isize - base;
                let count = vec.iter().filter(|x| **x >= tmp).count();
                vec.retain(|&x| x < tmp);
                ans.push(count);
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Option<usize>)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![(1, None), (2, Some(15)), (1, None), (3, Some(10)), (2, Some(20)), (3, Some(20))], vec![1, 1]),
            TestCase(15, vec![(1, None), (1, None), (2, Some(226069413)), (3, Some(1)), (1, None), (1, None), (2, Some(214168203)), (1, None), (3, Some(214168203)), (1, None), (1, None), (1, None), (2, Some(314506461)), (2, Some(245642315)), (3, Some(1))], vec![2, 2, 4]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
