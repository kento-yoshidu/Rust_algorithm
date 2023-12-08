// https://atcoder.jp/contests/abc123/tasks/abc123_b

pub fn run(a: usize, b: usize, c: usize, d: usize, e: usize) -> usize {
    let vec = vec![a, b, c, d, e];

    let mut ans = std::usize::MAX;

    for i in 0..5 {
        let mut total = 0;

        for (index, a) in vec.iter().enumerate() {
            if i != index {
                if a % 10 == 0 {
                    total += a;
                } else {
                    total += a + (10 - a % 10)
                }
            } else {
                total += a
            }
        }

        ans = ans.min(total);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(215, run(29, 20, 7, 35, 120));
        assert_eq!(481, run(101, 86, 119, 108, 57));
        assert_eq!(643, run(123, 123, 123, 123, 123));
    }
}
