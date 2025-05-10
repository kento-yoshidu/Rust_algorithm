// https://atcoder.jp/contests/abc106/tasks/abc106_b

fn check(n: usize) -> bool {
    let mut count = 0;

    for i in 1..=n {
        if n % i == 0 {
            count += 1;
        }
    }

    if count == 8 {
        true
    } else {
        false
    }
}

fn run(n: usize) -> usize {
    let mut ans = 0;

    for i in (1..=n).step_by(2) {
        if check(i) == true {
            ans += 1;
        }
    }

    ans
}

fn calc(n: usize) -> usize {
    (1..=n)
        .filter(|i| n % i == 0 )
        .count()
}

fn run2(n: usize) -> usize {
    (1..=n)
        .step_by(2)
        .filter(|num| calc(*num) == 8 )
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(104, 0),
            TestCase(105, 1),
            TestCase(134, 1),
            TestCase(135, 2),
            TestCase(164, 2),
            TestCase(165, 3),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
