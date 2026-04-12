// https://atcoder.jp/contests/abc289/tasks/abc289_b

fn run(n: usize, m: usize, a: Vec<usize>) -> Vec<usize> {
    if m == 0 {
        return (1..=n).collect();
    }

    let mut ans = Vec::new();
    let mut stack = Vec::new();

    for num in 1..=n {
        if a.contains(&num) {
            stack.push(num)
        } else {
            ans.push(num);

            for i in stack.iter().rev() {
                ans.push(*i);
            }

            stack.clear();
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc289_b() {
        let tests = [
            TestCase(5, 3, vec![1, 3, 4], vec![2, 1, 5, 4, 3]),
            TestCase(5, 0, vec![], vec![1, 2, 3, 4, 5]),
            TestCase(10, 6, vec![1, 2, 3, 7, 8, 9], vec![4, 3, 2, 1, 5, 6, 10, 9, 8, 7]),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
