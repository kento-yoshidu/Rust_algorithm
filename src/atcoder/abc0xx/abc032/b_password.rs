use std::collections::HashSet;

pub fn run(s: &str, k: usize) -> usize {
    let mut hash_set = HashSet::new();

    let chars: Vec<char> = s.chars().collect();

    for arr in chars.windows(k) {
        hash_set.insert(arr.iter().collect::<String>());
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run("abcabc", 2));
        assert_eq!(1, run("aaaaa", 1));
        assert_eq!(0, run("hello", 10));
    }
}
