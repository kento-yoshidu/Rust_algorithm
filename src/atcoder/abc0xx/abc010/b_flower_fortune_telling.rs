// https://atcoder.jp/contests/abc010/tasks/abc010_2

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in a {
        let mut num = i;

        loop {
            if num % 2 == 0 {
                ans += 1;
                num -= 1;
            } else if num % 3 == 2 {
                ans += 1;
                num -= 1;
            } else {
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
        assert_eq!(4, run(3, vec![5, 8, 2]));
        assert_eq!(8, run(9, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
}