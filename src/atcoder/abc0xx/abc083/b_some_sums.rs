// https://atcoder.jp/contests/abc083/tasks/abc083_b

fn calc(num: usize) -> usize {
    let mut n = num;
    let mut result = 0;

    while n > 0 {
        result += n % 10;
        n = n / 10;
    }

    result
}

fn run(n: usize, a: usize, b: usize) -> usize {
    let mut result = 0;

    for i in 1..=n {
        let res = calc(i);

        if a <= res && res <= b {
            result += i;
        }
    }

    result
}

fn rec(num: usize, total: usize) -> usize {
    if num == 0 {
        total
    } else {
        rec(num/10, total + num % 10)
    }
}

fn run2(n: usize, a: usize, b: usize) -> usize {
    (1..=n).filter(|num| {
        let res = rec(*num, 0);

        a <= res && res <= b
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(14, 2, 4, 45),
            TestCase(20, 2, 5, 84),
            TestCase(10, 1, 2, 13),
            TestCase(100, 4, 16, 4554),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
            assert_eq!(run2(n, a, b), expected);
        }
    }
}
