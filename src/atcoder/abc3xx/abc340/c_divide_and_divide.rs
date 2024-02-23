// https://atcoder.jp/contests/abc340/tasks/abc340_c

use std::collections::HashMap;

fn calc(n: usize, h: &mut HashMap<usize, usize>) -> usize {
    if n < 2 {
        return 0;
    }

    if let Some(x) = h.get(&n) {
        return *x;
    }

    let a = n/2;
    let b = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };

    let num = n + calc(a, h) + calc(b, h);
    h.entry(n).or_insert(num);

    return num;
}

pub fn run(n: usize) -> usize {
    let mut hash_map: HashMap<usize, usize> = HashMap::new();

    calc(n, &mut hash_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5),
            TestCase(340, 2888),
            TestCase(100000000000000000, 5655884811924144128),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
