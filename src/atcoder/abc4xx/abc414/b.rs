// https://atcoder.jp/contests/abc414/tasks/abc414_b

fn run(_n: usize, cl: Vec<(char, u128)>) -> String {
    let len: u128 = cl.iter().map(|(_, l)| l).sum();

    if len > 100 {
        return String::from("Too Long");
    }

    cl.into_iter()
        .map(|(c, l)| format!("{}", c.to_string().repeat(l as usize)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(char, u128)>, &'static str);

    #[test]
    fn abc414_b() {
        let tests = [
            TestCase(8, vec![('m', 1), ('i', 1), ('s', 2), ('i', 1), ('s', 2), ('i', 1), ('p', 2), ('i', 1)], "mississippi"),
            TestCase(7, vec![('a', 1000000000000000000), ('t', 1000000000000000000), ('c', 1000000000000000000), ('o', 1000000000000000000), ('d', 1000000000000000000), ('e', 1000000000000000000), ('r', 1000000000000000000)], "Too Long"),
            TestCase(1, vec![('a', 100)], "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            TestCase(6, vec![('g', 4), ('j', 1), ('m', 4), ('e', 4), ('d', 3), ('i', 4)], "ggggjmmmmeeeedddiiii"),
        ];

        for TestCase(n, cl, expected) in tests {
            assert_eq!(run(n, cl), expected);
        }
    }
}
