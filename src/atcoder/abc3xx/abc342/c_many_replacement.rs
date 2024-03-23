// https://atcoder.jp/contests/abc342/tasks/abc342_c

pub fn run(_n: usize, s: &str, _q: usize, cd: Vec<(char, char)>) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    let mut alphas: Vec<char> = ('a'..='z').collect();

    for (c, d) in cd {
        for i in 0..26 {
            if c == alphas[i] {
                alphas[i] = d;
            }
        }
    }

    for i in 0..chars.len() {
        chars[i] = alphas[chars[i] as usize - 'a' as usize];
    }

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize, Vec<(char, char)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, "atcoder", 4, vec![('r', 'a'), ('t', 'e'), ('d', 'v'), ('a', 'r')], "recover"),
            TestCase(3, "abc", 4, vec![('a', 'a'), ('s', 'k'), ('n', 'n'), ('z', 'b')], "abc"),
            TestCase(34, "supercalifragilisticexpialidocious", 20, vec![ ('g', 'c'), ('l', 'g'), ('g', 'm'), ('c', 'm'), ('r', 'o'), ('s', 'e'), ('a', 'a'), ('o', 'f'), ('f', 's'), ('e', 't'), ('t', 'l'), ('d', 'v'), ('p', 'k'), ('v', 'h'), ('x', 'i'), ('h', 'n'), ('n', 'j'), ('i', 'r'), ('s', 'i'), ('u', 'a')], "laklimamriiamrmrllrmlrkramrjimrial"),
        ];

        for TestCase(n, s, q, cd, expected) in tests {
            assert_eq!(run(n, s, q, cd), expected);
        }
    }
}
