// https://atcoder.jp/contests/abc300/tasks/abc300_a

pub fn run(n: i32, s: String) -> String {
    let mut t = 0;
    let mut a = 0;

    let mut ans = String::from("");

    for c in s.chars() {
        if c == 'A' {
            a += 1;
        } else {
            t += 1;
        }

        if t == (n + 1) / 2 {
            ans = String::from("T");
            break;
        }

        if a == (n + 1) / 2 {
            ans = String::from("A");
            break;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("T"), run(5, String::from("TTAAT")));
        assert_eq!(String::from("T"), run(6, String::from("ATTATA")));
        assert_eq!(String::from("A"), run(1, String::from("A")));
    }
}
