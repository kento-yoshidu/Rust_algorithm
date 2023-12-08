// https://atcoder.jp/contests/abc124/tasks/abc124_c

pub fn run(s: &str) -> usize {
    let ans = s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .fold([0, 0], |[l, r], (i, num)| {
            if i % 2 == num as usize {
                [l + 1, r]
            } else {
                [l, r + 1]
            }
        });

    *ans.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run("000"));
        assert_eq!(3, run("10010010"));
        assert_eq!(0, run("0101010101010101"));
        assert_eq!(0, run("1010101010101010"));
        assert_eq!(0, run("0"));
    }
}
