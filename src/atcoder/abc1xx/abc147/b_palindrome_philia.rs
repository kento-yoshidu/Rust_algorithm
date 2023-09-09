// https://atcoder.jp/contests/abc147/tasks/abc147_b

pub fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    (0..chars.len()/2).filter(|i| {
        chars[*i] != chars[s.len() - *i - 1]
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run("redcoder"));
        assert_eq!(0, run("wwwww"));
        assert_eq!(1, run("rng"));
        assert_eq!(50, run("ndfzvmkpudjeocebkfpexoszwczmpbdmivjnfeqapwvmbiiiarpwrjyezwdgydqbldyfyslboertiilckvacvroxycczmpfmdymu"));
        assert_eq!(10, run("aybmyzzankubfabovxfkoazziskrl"));
        assert_eq!(1, run("ax"));
        assert_eq!(0, run("xxx"));
        assert_eq!(34, run("uqoppvgpiqmsiwhpyfqnilmqkokdzowhrkzlavboipnljjlljpjwqalvxfvwpuairhxqiioqflgcwxvjupvghpadng"));
        assert_eq!(2, run("hjvqwycocvwqvth"));
        assert_eq!(34, run("xzamzvhfwhndreischtcucykbfjqasqlbkoxjpglbppptrvfccnfvlzppgdlmmseoidlqschqwnkfvqptsriiorvfqdjhrumjfc"));
    }
}
