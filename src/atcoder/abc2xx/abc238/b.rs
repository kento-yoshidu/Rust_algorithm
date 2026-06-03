// https://atcoder.jp/contests/abc238/tasks/abc238_b

fn run(_n: usize, a: Vec<isize>) -> isize {
    let mut vec = vec![0];

    let mut current = 0;

    for num in a {
        current += num;
        current %= 360;
        vec.push(current);
    }

    vec.push(360);
    vec.sort();

    vec.windows(2)
        .map(|arr| {
            arr[1] - arr[0]
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize,  Vec<isize>, isize);

    #[test]
    fn abc238_b() {
        let tests = [
            TestCase(4, vec![90, 180, 45, 195], 120),
            TestCase(1, vec![1], 359),
            TestCase(10, vec![215, 137, 320, 339, 341, 41, 44, 18, 241, 149], 170),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
