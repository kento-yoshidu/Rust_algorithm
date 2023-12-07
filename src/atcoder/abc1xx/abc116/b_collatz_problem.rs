// https://atcoder.jp/contests/abc116/tasks/abc116_b

use std::collections::HashSet;

pub fn run(s: usize) -> usize {
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

    #[test]
    fn test() {
        assert_eq!(5, run(8));
        assert_eq!(18, run(7));
        assert_eq!(114, run(54));
    }
}
