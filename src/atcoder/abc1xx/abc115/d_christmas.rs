// https://atcoder.jp/contests/abc115/tasks/abc115_d

fn func(n: usize, x: usize) -> usize {
    if n == 0 {
        return 1;
    } else {
        let length = (1 << n+1) - 3;
        let num = (1 << n) - 1;

        if x == 1 {
            return 0;
        } else if x <= length+1 {
            return  func(n-1, x-1);
        } else if x == length + 2 {
            return  num + 1;
        } else if x <= (length+1)*2 {
            return num + 1 + func(n-1, x-length-2);
        } else {
            return num * 2 + 1;
        }
    }
}

fn run(n: usize, x: usize) -> usize {
    func(n, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 7, 4),
            TestCase(1, 1, 0),
            TestCase(50, 4321098765432109, 2160549382716056),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
