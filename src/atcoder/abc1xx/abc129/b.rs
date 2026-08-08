// https://atcoder.jp/contests/abc129/tasks/abc129_b

fn run(n: isize, vec: &Vec<isize>) -> isize {
    let mut ans = 100;

    for i in 1..n {
        let mut l = 0;
        let mut r = 0;

        for (index, v) in vec.iter().enumerate() {
            if i >= index.try_into().unwrap() {
                l += v
            } else {
                r += v
            }
        }

        ans = ans.min(((l-r) as isize).abs())
    }

    ans
}

fn run2(n: isize, vec: &Vec<isize>) -> isize {
    let mut ans = 100;

    for i in 1..n {
        let mut tmp = 0;

        for (index, v) in vec.iter().enumerate() {
            if i > index.try_into().unwrap() {
                tmp += v;
            } else {
                tmp -= v;
            }
        }

        ans = ans.min(tmp.abs());
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, Vec<isize>, isize);

    #[test]
    fn abc129_b() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 0),
            TestCase(4, vec![1, 3, 1, 1], 2),
            TestCase(8, vec![27, 23, 76, 2, 3, 5, 62, 52], 2),
        ];

        for TestCase(n, vec, expected) in tests {
            assert_eq!(run(n, &vec), expected);
            assert_eq!(run2(n, &vec), expected);
        }
    }
}
