// https://atcoder.jp/contests/abc216/tasks/abc216_c

pub fn run(n: isize) -> String {
    let mut ans = String::new();

    let mut num = n;

    loop {
        if num == 0 {
            break;
        } else if num % 2 == 0 {
            num /= 2;
            ans.push('B');
        } else {
            num -= 1;
            ans.push('A');
        }
    }

    ans.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABBA"), run(5));
        assert_eq!(String::from("ABABAB"), run(14));
    }
}
