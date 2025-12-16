// https://atcoder.jp/contests/abc249/tasks/abc249_a

use core::cmp::min;

fn calc(run: usize, speed: usize, rest: usize, x: usize) -> usize {
    let q = x / (run + rest);

    run * speed * q + min(run, x - (run + rest) * q) * speed
}

fn run(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, x: usize) -> &'static str {
    let takahashi = calc(a, b, c, x);
    let aoki = calc(d, e, f, x);

    if takahashi > aoki {
        "Takahashi"
    } else if aoki > takahashi {
        "Aoki"
    } else {
        "Draw"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize, usize, usize, &'static str);

    #[test]
    fn abc249_a() {
        let tests = [
            TestCase(4, 3, 3, 6, 2, 5, 10, "Takahashi"),
            TestCase(3, 1, 4, 1, 5, 9, 2, "Aoki"),
            TestCase(1, 1, 1, 1, 1, 1, 1, "Draw"),
        ];

        for TestCase(a, b, c, d, e, f, x, excpected) in tests {
            assert_eq!(run(a, b, c, d, e, f, x), excpected);
        }
    }
}
