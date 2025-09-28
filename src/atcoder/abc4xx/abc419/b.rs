// https://atcoder.jp/contests/abc419/tasks/abc419_b

fn run(_q: usize, query: Vec<(usize, Option<usize>)>) -> Vec<usize> {
    let mut arr = vec![0; 101];

    let mut ans = Vec::new();

    for (q, x)in query {
        match q {
            1 => {
                arr[x.unwrap()] += 1;
            },
            2 => {
                for i in 0..100 {
                    if arr[i] > 0 {
                        ans.push(i);
                        arr[i] -= 1;
                        break;
                    }
                }
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
    fn abc419_b() {
        let tests = [
            TestCase(5, vec![(1, Some(6)), (1, Some(7)), (2, None), (1, Some(1)), (2, None)], vec![6, 1]),
            TestCase(8, vec![(1, Some(5)), (1, Some(1)), (1, Some(1)), (1, Some(9)), (2, None), (2, None), (1, Some(2)), (2, None)], vec![1, 1, 2]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
