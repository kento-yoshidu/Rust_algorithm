// https://atcoder.jp/contests/abc171/tasks/abc171_a

pub fn run(a: char) -> char {
    if a.is_uppercase() {
        'A'
    } else {
        'a'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('A', run('B'));
        assert_eq!('a', run('a'));
    }
}
