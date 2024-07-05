// https://atcoder.jp/contests/arc010/tasks/arc010_1

pub fn run(n: usize, m: usize, a: usize, b: usize, c: Vec<usize>) -> String {
    let mut cur = n;

    for i in 0..m {
        if cur <= a {
            cur += b;
        }

        if cur < c[i] {
            return (i+1).to_string();
        } else {
            cur -= c[i];
        }
    }

    String::from("complete")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(100, 3, 0, 100, vec![10, 20, 30], "complete"),
            TestCase(100, 4, 0, 100, vec![10, 20, 30, 40], "complete"),
            TestCase(100, 4, 0, 100, vec![50, 40, 30, 20], "3"),
            TestCase(100, 4, 10, 100, vec![50, 40, 30, 20], "complete"),
            TestCase(5, 3, 20, 10, vec![15, 5, 20], "3"),
        ];

        for TestCase(n, m, a, b, c, expected) in tests {
            assert_eq!(run(n, m, a, b, c), expected);
        }
    }
}
