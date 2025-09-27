// https://atcoder.jp/contests/abc147/tasks/abc147_b

fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    (0..chars.len()/2)
        .filter(|i| {
            chars[*i] != chars[s.len() - *i - 1]
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc147_b() {
        let tests = [
            TestCase("redcoder", 1),
            TestCase("wwwww", 0),
            TestCase("rng", 1),
            TestCase("ndfzvmkpudjeocebkfpexoszwczmpbdmivjnfeqapwvmbiiiarpwrjyezwdgydqbldyfyslboertiilckvacvroxycczmpfmdymu", 50),
            TestCase("aybmyzzankubfabovxfkoazziskrl", 10),
            TestCase("ax", 1),
            TestCase("xxx", 0),
            TestCase("uqoppvgpiqmsiwhpyfqnilmqkokdzowhrkzlavboipnljjlljpjwqalvxfvwpuairhxqiioqflgcwxvjupvghpadng", 34),
            TestCase("hjvqwycocvwqvth", 2),
            TestCase("xzamzvhfwhndreischtcucykbfjqasqlbkoxjpglbppptrvfccnfvlzppgdlmmseoidlqschqwnkfvqptsriiorvfqdjhrumjfc", 34),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
