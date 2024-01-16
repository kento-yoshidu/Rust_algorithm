// https://atcoder.jp/contests/abc001/tasks/abc001_1

pub fn run(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    struct TestCase(i32, i32, i32);

    #[rstest]
    #[case::c_1st(15, 10, 5)]
    #[case::c_2nd(0, 0, 0)]
    #[case::c_3rd(5, 20, -15)]
    fn test(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(expected, run(a, b));
    }
}
