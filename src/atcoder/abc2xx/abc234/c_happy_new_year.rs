// https://atcoder.jp/contests/abc234/tasks/abc234_c

pub fn run(k: usize) -> String {
    let s = format!("{:b}", k);

    s.chars()
        .map(|c| {
            if c == '1' {
                '2'
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("22"), run(3));
        assert_eq!(String::from("2022"), run(11));
        assert_eq!(String::from("220022020000202020002022022000002020002222002200002022002200"), run(923423423420220108));
    }
}
