// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_d

#[allow(dead_code)]
pub fn run(n: String) -> i32 {
    let mut ans = 0;

    for (i, c) in n.chars().rev().enumerate() {
        let num = (c as i32) - 48;

        ans += num * (1 << i);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(13, run(String::from("1101")));
    }
}
