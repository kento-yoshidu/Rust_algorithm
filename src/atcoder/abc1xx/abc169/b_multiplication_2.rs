// https://atcoder.jp/contests/abc169/tasks/abc169_b

fn run(_n: usize, a: &Vec<i128>) -> i128 {
    if a.contains(&0) {
        return 0
    }

    let mut ans: i128 = 1;

    for num in a {
        if ans * num > 10_i128.pow(18_u32) {
            ans = -1;
            break;
        } else {
            ans *= num;
        }
    }

    ans as i128
}

fn run2(_n: usize, a: &Vec<i128>) -> i128 {
    a.into_iter()
        .fold(1, |state, num| {
            if state * num > 10_usize.pow(18_u32) as i128 {
                -1
            } else {
                state * num
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<i128>, i128);

    #[test]
    fn abc169_b() {
        let tests = [
            TestCase(2, vec![1000000000, 1000000000], 1000000000000000000),
            TestCase(3, vec![101, 9901, 999999000001], -1),
            TestCase(31, vec![4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
