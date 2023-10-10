// https://atcoder.jp/contests/abc002/tasks/abc002_2

pub fn run(s: String) -> String {
    s.chars().filter(|c| {
        !"aiueo".contains(*c)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("chkd"), run(String::from("chokudai")));
        assert_eq!(String::from("knmch"), run(String::from("okanemochi")));
        assert_eq!(String::from("k"), run(String::from("aoki")));
        assert_eq!(String::from("mzsh"), run(String::from("mazushii")));
    }
}

