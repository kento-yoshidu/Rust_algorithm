// https://atcoder.jp/contests/abc246/tasks/abc246_a

fn run(vec: [(i32, i32); 3]) -> (i32, i32) {
    vec.into_iter()
        .fold((0, 0), |a, b| {
            (a.0 ^ b.0, a.1 ^ b.1)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([(i32, i32); 3], (i32, i32));

    #[test]
    fn test() {
        let tests = [
            TestCase([(-1, -1), (-1, 2), (3, 2)], (3, -1)),
            TestCase([(-60, -40), (-60, -80), (-20, -80)], (-20, -40)),
        ];

        for TestCase(vec, expected) in tests {
            assert_eq!(run(vec), expected);
        }
    }
}
