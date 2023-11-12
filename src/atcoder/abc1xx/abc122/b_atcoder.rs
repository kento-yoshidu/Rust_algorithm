// https://atcoder.jp/contests/abc122/tasks/abc122_b

pub fn run(str: String) -> i32 {
    let mut len = 0;
    let mut max_len = 0;

    for c in str.chars() {
        if "ACGT".contains(c) {
            len += 1;
            max_len = max_len.max(len);
        } else {
            len = 0;
        }
    }

    max_len
}

pub fn run2(str: &str) -> usize {
    let chars: Vec<char> = str.chars().collect();

    chars.iter()
        .fold((0, 0), |(ans, state), c| {
            if "ACGT".contains(*c) {
                (ans.max(state+1), state+1)
            } else {
                (ans, 0)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(String::from("ATCODER")));
        assert_eq!(5, run(String::from("HATAGAYA")));
        assert_eq!(0, run(String::from("SHINJUKU")));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2("ATCODER"));
        assert_eq!(5, run2("HATAGAYA"));
        assert_eq!(0, run2("SHINJUKU"));
    }
}
