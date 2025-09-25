// https://atcoder.jp/contests/abc130/tasks/abc130_b

fn run(_n: usize, x: usize, vec: &Vec<usize>) -> usize {
    let mut current = 0;
    let mut ans = 0;

    for i in vec.iter() {
        if current <= x {
            current += i;
            ans += 1;
        } else {
            break;
        }
    }

    ans
}

fn run2(_n: usize, x: usize, l: &Vec<usize>) -> usize {
    l.iter()
        .scan(0, |state, d| {
            *state += d;
            Some(*state)
        })
        .filter(|d| {
            *d <= x
        })
        .count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc130_b() {
        let tests = [
            TestCase(3, 6, vec![3, 4, 5], 2),
            TestCase(4, 9, vec![3, 3, 3, 3], 4),
        ];

        for TestCase(n, x, l, expected) in tests {
            assert_eq!(run(n, x, &l), expected);
            assert_eq!(run2(n, x, &l), expected);
        }
    }
}
