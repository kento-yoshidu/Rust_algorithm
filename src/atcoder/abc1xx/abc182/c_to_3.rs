// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run(n: &str) -> isize {
    let mut ans = std::isize::MAX;

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

    if ans == std::isize::MAX {
        -1
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, isize);

    #[test]
    fn abc182_c() {
        let tests = [
            TestCase("35", 1),
            TestCase("369", 0),
            TestCase("6227384", 1),
            TestCase("11", -1),
            TestCase("641", 2),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
