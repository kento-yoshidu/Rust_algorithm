// https://atcoder.jp/contests/abc249/tasks/abc249_a

use core::cmp::min;

pub fn calc(run: usize, speed: usize, rest: usize, x: usize) -> usize {
    let q = x / (run + rest);

    run * speed * q + min(run, x - (run + rest) * q) * speed
}

pub fn run(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, x: usize) -> String {
    let takahashi = calc(a, b, c, x);
    let aoki = calc(d, e, f, x);

    if takahashi > aoki {
        String::from("Takahashi")
    } else if aoki > takahashi {
        String::from("Aoki")
    } else {
        String::from("Draw")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Takahashi"), run(4, 3, 3, 6, 2, 5, 10));
        assert_eq!(String::from("Aoki"), run(3, 1, 4, 1, 5, 9, 2));
        assert_eq!(String::from("Draw"), run(1, 1, 1, 1, 1, 1, 1));
    }
}
