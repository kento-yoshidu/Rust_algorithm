// https://atcoder.jp/contests/abc095/tasks/abc095_b

fn run(n: i32, x: i32, vec: &Vec<i32>) -> i32 {
    let minimum: i32 = vec.iter().sum();

    let rest = x - minimum;

    let mut ans = n;

    if rest != 0 {
        let min = vec.iter().min().unwrap();

        ans += rest / min;
    }

    ans
}

fn run2(n: i32, x: i32, m: &Vec<i32>) -> i32 {
    let min = m.iter().min().unwrap();
    let sum: i32 = m.iter().sum();
    let rest = x - sum;

    n + rest / min
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, Vec<i32>, i32);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 1000, vec![120, 100, 140], 9),
            TestCase(4, 360, vec![90, 90, 90, 90], 4),
            TestCase(5, 3000, vec![150, 130, 150, 130, 110], 26),
        ];

        for TestCase(n, x, m, expected) in tests {
            assert_eq!(run(n, x, &m), expected);
            assert_eq!(run2(n, x, &m), expected);
        }
    }
}
