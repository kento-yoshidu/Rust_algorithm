// https://atcoder.jp/contests/abc351/tasks/abc351_c

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .fold(Vec::new(), |mut stack, num| {
            stack.push(num);

            loop {
                let len = stack.len();

                if len > 1 && stack[len-2] == stack[len-1] {
                    let i = stack[len-1];

                    stack.truncate(len-2);

                    stack.push(i+1);
                } else {
                    break stack;
                }
            }
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7 , vec![2, 1, 1, 3, 5, 3, 3], 3),
            TestCase(5, vec![0, 0, 0, 1, 2], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
