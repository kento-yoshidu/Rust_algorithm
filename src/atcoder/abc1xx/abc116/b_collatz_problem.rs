// https://atcoder.jp/contests/abc116/tasks/abc116_b

use std::collections::HashSet;

fn run(s: usize) -> usize {
    let mut ans = 0;
    let mut num = s;

    let mut hashset: HashSet::<usize> = HashSet::new();

    loop {
        ans += 1;

        if let Some(_) = &hashset.get(&num) {
            return ans
        };

        hashset.insert(num);

        if num % 2 == 0 {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc116_b() {
        let tests = [
            TestCase(8, 5),
            TestCase(7, 18),
            TestCase(54, 114),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
