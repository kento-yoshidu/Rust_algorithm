// https://atcoder.jp/contests/abc220/tasks/abc220_a

pub fn run(a: isize, b: isize, c: isize) -> isize {
    (a..=b).find(|num| {
        num % c == 0
    }).unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(200, run(123, 456, 100));
        assert_eq!(-1, run(630, 940, 314));
    }
}
