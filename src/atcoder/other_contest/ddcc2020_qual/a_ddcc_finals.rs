// https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_a

fn calc(n: usize) -> usize {
    match n {
        1 => {
            300000
        },
        2 => {
            200000
        },
        3 => {
            100000
        },
        _ => 0
    }
}

fn run(x: usize, y: usize) -> usize {
    if x == 1 && y == 1 {
        return 1000000;
    }

    let mut ans = 0;

    ans += calc(x);
    ans += calc(y);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 1, 1000000),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
