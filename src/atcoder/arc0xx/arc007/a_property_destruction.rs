// https://atcoder.jp/contests/arc007/tasks/arc007_1

pub fn run(x: char, s: String) -> String {
    s.chars().filter(|c| {
        *c != x
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("bcdefgbcdefg", run('a', String::from("abcdefgabcdefg")));
        assert_eq!("aassddff", run('g', String::from("aassddffgg")));
        assert_eq!("", run('a', String::from("aaaaa")));
    }
}
