// https://atcoder.jp/contests/abc042/tasks/abc042_a

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize) -> &'static str {
    let vec = vec![a, b, c];

    let mut five = 0;
    let mut seven = 0;

    for i in vec.iter() {
        match i {
            5 => five += 1,
            7 => seven += 1,
            _ => continue
        }
    }

    if five == 2 && seven == 1 {
        "YES"
    } else {
        "NO"
    }
}

fn run2(a: usize, b: usize, c: usize) -> &'static str {
    if a * b * c == 5 * 7 * 5 {
        "YES"
    } else {
        "NO"
    }
}

fn run3(a: usize, b: usize, c: usize) -> &'static str {
    if vec![a, b, c].into_iter().sorted().collect::<Vec<usize>>() == [5, 5, 7] {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 5, 7, "YES"),
            TestCase(7, 7, 5, "NO"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
            assert_eq!(run3(a, b, c), expected);
        }
    }
}
