// https://atcoder.jp/contests/arc006/tasks/arc006_1

pub fn run(e: [usize; 6], b: usize, l: [usize; 6]) -> usize {
    let mut ans = 0;

    for num in l {
        if e.contains(&num) {
            ans += 1;
        }
    }

    if ans == 6 {
        return 1;
    }

    if ans == 5 {
        if l.contains(&b) {
            2
        } else {
            3
        }
    } else if ans == 4 {
        4
    } else if ans == 3 {
        5
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 6], usize, [usize; 6], usize);

    #[test]
    fn test() {
        let tests = [
            TestCase([1, 2, 3, 4, 5, 6], 7, [1, 2, 3, 4, 5, 6], 1),
            TestCase([0, 1, 3, 5, 7, 9], 7, [0, 2, 4, 6, 8, 9], 0),
            TestCase([0, 2, 6, 7, 8, 9], 4, [0, 5, 6, 7, 8, 9], 3),
            TestCase([1, 3, 5, 6, 7, 8], 9, [3, 5, 6, 7, 8, 9], 2),
            TestCase([0, 1, 3, 4, 5, 7], 8, [2, 3, 5, 7, 8, 9], 5),
        ];

        for TestCase(e, b, l, expected) in tests {
            assert_eq!(run(e, b, l), expected);
        }
    }
}
