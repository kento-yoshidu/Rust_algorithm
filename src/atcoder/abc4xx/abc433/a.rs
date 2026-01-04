// https://atcoder.jp/contests/abc433/tasks/abc433_a

fn run(x: usize, y: usize, z: usize) -> &'static str {
    let mut x = x;
    let mut y = y;

    loop {
        if y*z > x {
            return "No";
        }

        if y*z == x {
            return "Yes";
        }

        x += 1;
        y += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc433_a() {
        let tests = [
            TestCase(44, 20, 2, "Yes"),
            TestCase(28, 10, 3, "No"),
            TestCase(50, 5, 10, "Yes"),
            TestCase(1, 100, 2, "No")
        ];

        for TestCase(x, y, z, expected) in tests {
            assert_eq!(run(x, y, z), expected);
        }
    }
}
