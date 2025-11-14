// https://atcoder.jp/contests/abc206/tasks/abc206_b

fn run(num: isize) -> isize {
    let mut total = 0;
    let mut day = 0;

    for i in 1..=num {
        if total < num {
            total += i;
            day += 1;
        } else {
            break;
        }
    }

    day
}

fn check(count: isize, rest: isize) -> isize {
    if rest - count <= 0 {
        count
    } else {
        check(count+1, rest - count)
    }
}

fn run2(n: isize) -> isize {
    check(1, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn abc206_b() {
        let tests = [
            TestCase(12, 5),
            TestCase(100128, 447),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
