// https://atcoder.jp/contests/abc351/tasks/abc351_b

pub fn run(_n: usize, a: Vec<&str>, b: Vec<&str>) -> (usize, usize) {
    let a_vec: Vec<Vec<char>> = a.iter().map(|str| str.chars().collect()).collect();
    let b_vec: Vec<Vec<char>> = b.iter().map(|str| str.chars().collect()).collect();

    for (i, (a, b)) in a_vec.iter().zip(b_vec).enumerate() {
        for (j, (x, y)) in a.iter().zip(b).enumerate() {
            if *x != y {
                return (i+1, j+1);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["abc", "def", "ghi"], vec!["abc", "bef", "ghi"], (2, 1)),
            TestCase(1, vec!["f"], vec!["q"], (1, 1)),
            TestCase(10, vec!["eixfumagit", "vtophbepfe", "pxbfgsqcug", "ugpugtsxzq", "bvfhxyehfk", "uqyfwtmglr", "jaitenfqiq", "acwvufpfvv", "jhaddglpva", "aacxsyqvoj"],
                            vec!["eixfumagit", "vtophbepfe", "pxbfgsqcug", "ugpugtsxzq", "bvfhxyehok", "uqyfwtmglr", "jaitenfqiq", "acwvufpfvv", "jhaddglpva", "aacxsyqvoj)"], (5, 9)),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
