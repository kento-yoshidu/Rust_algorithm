// https://atcoder.jp/contests/abc276/tasks/abc276_a

fn run(s: &str) -> isize {
    let mut result = -1;

    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            result = i as isize + 1;
        }
    }

    result
}

fn run2(s: &str) -> isize {
    s
        .rfind('a')
        .map(|i| (i + 1) as isize)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, isize);

    #[test]
    fn abc276_a() {
        let tests = [
            TestCase("abcdaxayz", 7),
            TestCase("bcbbbz", -1),
            TestCase("aaaaa", 5),
            TestCase("lpixtnpukwdrlfjzkipzdqxiptrmkwtyqazyifpwpwqhebycyrnsnmwfqgghhemrqbxcmrwfqyyhczksgblxasbiqkrpniybvevw", 85),
            TestCase("ueqonvbdgvodlguqnbuqklnvrlfbhogwkrdmxwiqhscpmxxnnnlvoribrpkvnsqmvpcqrotzzgidiebptnryogxodezjwjdhofgs", -1),
            TestCase("eqeahywzyrllfbqdvemfwqekmzjbg", 4),
            TestCase("kyxgeysxtpjbfzweqlb", -1),
            TestCase("ahxywxnihxjjragzylkswjahahcbzprzvxciod", 25),
            TestCase("cntfuvmulcmuwrpnhyuufyclguggfihexwmehbohhqgiphroufkdjnhqmsjqvprbygilrvqr", -1),
            TestCase("durozujwjwhh", -1),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
