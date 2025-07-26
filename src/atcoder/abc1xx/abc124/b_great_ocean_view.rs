// https://atcoder.jp/contests/abc124/tasks/abc124_b

fn run(_n: usize, vec: &Vec<usize>) -> usize {
    let mut height = vec[0];

    let mut ans = 0;

    for h in vec.into_iter() {
        if height <= *h {
            ans += 1;
            height = *h;
        }
    }

    ans
}

fn run2(_n: usize, vec: &Vec<usize>) -> usize {
    vec.into_iter()
        .fold((0, 0), |(count, state), height| {
            if state <= *height {
                (count + 1, *height)
            } else {
                (count, state)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc124_b() {
        let tests = [
            TestCase(4, vec![6, 5, 6, 8], 3),
            TestCase(5, vec![4, 5, 3, 5, 4], 3),
            TestCase(5, vec![9, 5, 6, 8, 4], 1),
            TestCase(5, vec![1, 2, 3, 9, 4], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
