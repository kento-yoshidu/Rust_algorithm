// https://atcoder.jp/contests/abc162/tasks/abc162_b

fn fizzbuzz(num: usize) -> bool {
    if !(num % 3 == 0 || num % 5 == 0) {
        return true;
    };

    false
}

fn run(num: usize) -> usize {
    let mut total = 0;

    for i in 1..=num {
        if fizzbuzz(i) {
            total += i;
        }
    }

    total
}

fn run2(num: usize) -> usize {
    (1..=num)
        .filter(|n| {
            !(n % 3 == 0 || n % 5 == 0)
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc162_b() {
        let tests = [
            TestCase(15, 60),
            TestCase(1000000, 266666333332),
        ];

        for TestCase(num, expected) in tests {
            assert_eq!(run(num), expected);
            assert_eq!(run2(num), expected);
        }
    }
}
