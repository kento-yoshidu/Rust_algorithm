// https://atcoder.jp/contests/abc097/tasks/abc097_b

#[allow(dead_code)]
pub fn run(n: i32) -> i32 {
    let mut ans = 1;

    for i in 2..1000 {
        let mut j = 2;

        if i > n {
            break
        }

        loop {
            let num = (i as i32).pow(j);

            if num > n {
                break
            }  else {
                ans = ans.max(num);
                j += 1;
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
        assert_eq!(9, run(10));
        assert_eq!(1, run(1));
        assert_eq!(961, run(999));
    }
}
