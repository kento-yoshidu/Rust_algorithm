// https://atcoder.jp/contests/abc264/tasks/abc264_b

use std::cmp::max;

fn run(r: isize, c: isize) -> &'static str {
    let dis = max((r-8).abs(), (c-8).abs());

    if dis % 2 == 0 {
        "white"
    } else {
        "black"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, &'static str);

    #[test]
    fn abc264_b() {
        let tests = [
            TestCase(3, 5, "black"),
            TestCase(4, 5, "white"),
        ];

        for TestCase(r, c, expected) in tests {
            assert_eq!(run(r, c), expected);
        }
    }
}
