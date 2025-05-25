// https://atcoder.jp/contests/abc108/tasks/abc108_b

fn run(p1: (i32, i32), p2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dy = p2.0 - p1.0;
    let dx = p2.1 - p1.1;

    let p3 = (p2.0 - dx, p2.1 + dy);
    let p4 = (p1.0 - dx, p1.1 + dy);

    (p3, p4)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase((i32, i32), (i32, i32), ((i32, i32), (i32, i32)));

    #[test]
    fn abc108_b() {
        let tests = [
            TestCase((0, 0), (0, 1), ((-1, 1), (-1, 0))),
            TestCase((2, 3), (6, 6), ((3, 10), (-1, 7))),
            TestCase((31, -41),  (-59, 26), ((-126, -64), (-36, -131))),
        ];

        for TestCase(p1, p2, expected) in tests {
            assert_eq!(run(p1, p2), expected);
        }
    }
}
