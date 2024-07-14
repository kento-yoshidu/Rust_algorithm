// https://atcoder.jp/contests/abc362/tasks/abc362_b

pub fn run(a: (isize, isize), b: (isize, isize), c: (isize, isize)) -> &'static str {
    let ab = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2);
    let bc = (b.0 - c.0).pow(2) + (b.1 - c.1).pow(2);
    let ac = (a.0 - c.0).pow(2) + (a.1 - c.1).pow(2);

    if ab + bc == ac ||
        ab + ac == bc ||
        bc + ac == ab {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase((isize, isize), (isize, isize), (isize, isize), &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase((0, 0), (4, 0), (0, 3), "Yes"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
