// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run(n: &str) -> i32 {
    let mut ans = std::i32::MAX;

    if n.parse::<usize>().unwrap() % 3 == 0 {
        return 0;
    }

    for bit in 1..(1 << n.len()) {
        let mut num = String::new();

        // bit全探索で数値を選び、i32に変換する
        for i in 0..n.len() {
            if bit & (1 << i) != 0 {
                num.push(n.chars().nth(i).unwrap());
            }
        }

        // 何文字削除したか
        let delete_count = n.len() - num.len();

        if num.parse::<usize>().unwrap() % 3 == 0 {
            ans = ans.min(delete_count.try_into().unwrap());
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
        assert_eq!(2, run("641"));
    }
}
