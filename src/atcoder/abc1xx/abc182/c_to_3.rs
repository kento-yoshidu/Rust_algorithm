// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run(n: &str) -> i32 {
    let mut ans = std::i32::MAX;

    if n.parse::<usize>().unwrap() % 3 == 0 {
        return 0;
    }

    for bit in 1..(1 << n.len()) {
        let mut num = 0;
        let mut count = 0;

        // bit全探索で数値を選び、i32に変換する
        for i in 0..n.len() {
            if bit & (1 << i) != 0 {
                num += (n.chars().nth(i).unwrap() as i32 - 48) * (10 as f64).powf(i as f64) as i32;
                count += 1;
            }
        }

        if num % 3 == 0 {
            ans = ans.min(count);
        }
    }

    if ans == std::i32::MAX {
        -1
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run("35"));
        assert_eq!(0, run("369"));
        assert_eq!(1, run("6227384"));
        assert_eq!(-1, run("11"));
    }
}
