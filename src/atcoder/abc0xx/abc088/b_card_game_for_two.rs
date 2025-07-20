// https://atcoder.jp/contests/abc088/tasks/abc088_b

fn run(_n: i32, vec: &mut Vec<i32>) -> i32 {
    vec.sort();

    let mut ans = 0;

    for (i, num) in vec.iter().rev().enumerate() {
        if i % 2 == 0 {
            ans += num
        } else {
            ans -= num
        }
    }

    ans
}

fn run2(_n: isize, vec: &mut Vec<isize>) -> isize {
    vec.sort();

    vec
        .iter()
        .rev()
        .enumerate()
        .map(|(i, num)| {
            if i % 2 == 0 {
                *num
            } else {
                -num
            }
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, &mut vec![3, 1], 2),
            TestCase(2, &mut vec![2, 7, 4], 5),
            TestCase(2, &mut vec![20, 18, 2, 18], 18),
        ];

        for TestCase(n, vec, expected) in tests {
            assert_eq!(run(n, &mut vec), expected);
            assert_eq!(run2(n, &mut vec), expected);
        }
    }
}
