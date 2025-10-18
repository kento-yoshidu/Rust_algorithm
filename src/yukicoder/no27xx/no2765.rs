// https://yukicoder.me/problems/no/2765

fn run(a: (isize, isize, isize), b: (isize, isize, isize)) -> [isize; 3] {
    let x = a.1 * b.2 - a.2 * b.1;
    let y = a.2 * b.0 - a.0 * b.2;
    let z = a.0 * b.1 - a.1 * b.0;

    [x, y, z]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase((isize, isize, isize), (isize, isize, isize), [isize; 3]);

    #[test]
    fn yuki_2765() {
        let tests = [
            TestCase((3, 1, 4), (2, 7, 1), [-27, 5, 19]),
            TestCase((314, 159, 265), (271, 828, 182), [-190482, 14667, 216903]),
            TestCase((27, -1828, 1828), (-45, 90, -45), [-82260, -81045, -79830]),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
