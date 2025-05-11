// https://atcoder.jp/contests/abc405/tasks/abc405_a

fn run(r: usize, x: usize) -> &'static str {
    match x {
        1 => {
            if 1600 <= r && r <= 2999 {
                "Yes"
            } else {
                "No"
            }
        },
        2 => {
            if 1200 <= r && r <= 2399 {
                "Yes"
            } else {
                "No"
            }
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc405_a() {
        let tests = [
            TestCase(2000, 1, "Yes"),
            TestCase(1000, 1, "No"),
            TestCase(1500, 2, "Yes"),
            TestCase(2800, 2, "No"),
        ];

        for TestCase(r, x, expected) in tests {
            assert_eq!(run(r, x), expected);
        }
    }
}
