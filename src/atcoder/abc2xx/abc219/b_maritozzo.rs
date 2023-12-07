// https://atcoder.jp/contests/abc219/tasks/abc219_b

pub fn run(s: Vec<&str>, t: &str) -> String {
    let nums: Vec<usize> =
        t.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    nums.iter()
        .map(|i| {
            s[i-1]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("marizzotomari"), run(vec!["mari", "to", "zzo"], "1321"));
        assert_eq!(String::from("abracadabra"), run(vec!["abra", "cad", "abra"], "123"));
        assert_eq!(String::from("a"), run(vec!["a", "b", "c"], "1"));
    }
}
