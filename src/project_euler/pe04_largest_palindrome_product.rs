pub fn check(n: usize) -> bool {
    n.to_string().chars().eq(n.to_string().chars().rev())
}

pub fn run() -> usize {
    let mut ans= 0;

    for i in 100..1000 {
        for j in (i..1000).rev() {
            if i % 11 != 0 && j % 11 != 0 {
                continue
            }

            if check(i * j) {
                ans = ans.max(i * j);
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(906609, run());
    }
}

// https://gleamath.com/number-of-palindromes/
