// https://atcoder.jp/contests/abc276/tasks/abc276_a

pub fn run(input: &str) -> i8 {
    let mut result: i8 = -1;

    for (i, c) in input.chars().enumerate() {
        if c == 'a' {
            result = i as i8 + 1;
        }
    }

    result
}

pub fn run2(s: String) -> isize {
    s
        .rfind('a')
        .map(|i| (i + 1) as isize)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("abcdaxayz"), 7);
        assert_eq!(run("bcbbbz"), -1);
        assert_eq!(run("aaaaa"), 5);
        assert_eq!(85, run("lpixtnpukwdrlfjzkipzdqxiptrmkwtyqazyifpwpwqhebycyrnsnmwfqgghhemrqbxcmrwfqyyhczksgblxasbiqkrpniybvevw"));
        assert_eq!(-1, run("ueqonvbdgvodlguqnbuqklnvrlfbhogwkrdmxwiqhscpmxxnnnlvoribrpkvnsqmvpcqrotzzgidiebptnryogxodezjwjdhofgs"));
        assert_eq!(4, run("eqeahywzyrllfbqdvemfwqekmzjbg"));
        assert_eq!(-1, run("kyxgeysxtpjbfzweqlb"));
        assert_eq!(25, run("ahxywxnihxjjragzylkswjahahcbzprzvxciod"));
        assert_eq!(-1, run("cntfuvmulcmuwrpnhyuufyclguggfihexwmehbohhqgiphroufkdjnhqmsjqvprbygilrvqr"));
        assert_eq!(-1, run("durozujwjwhh"));
    }

    #[test]
    fn test2() {
        assert_eq!(run2(String::from("abcdaxayz")), 7);
        assert_eq!(run2(String::from("bcbbbz")), -1);
        assert_eq!(run2(String::from("aaaaa")), 5);
        assert_eq!(85, run2(String::from("lpixtnpukwdrlfjzkipzdqxiptrmkwtyqazyifpwpwqhebycyrnsnmwfqgghhemrqbxcmrwfqyyhczksgblxasbiqkrpniybvevw")));
        assert_eq!(-1, run2(String::from("ueqonvbdgvodlguqnbuqklnvrlfbhogwkrdmxwiqhscpmxxnnnlvoribrpkvnsqmvpcqrotzzgidiebptnryogxodezjwjdhofgs")));
        assert_eq!(4, run2(String::from("eqeahywzyrllfbqdvemfwqekmzjbg")));
        assert_eq!(-1, run2(String::from("kyxgeysxtpjbfzweqlb")));
        assert_eq!(25, run2(String::from("ahxywxnihxjjragzylkswjahahcbzprzvxciod")));
        assert_eq!(-1, run2(String::from("cntfuvmulcmuwrpnhyuufyclguggfihexwmehbohhqgiphroufkdjnhqmsjqvprbygilrvqr")));
        assert_eq!(-1, run2(String::from("durozujwjwhh")));
    }
}
