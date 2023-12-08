// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run<'a>(n: usize, k: usize, s: Vec<&'a str>) -> Vec<&'a str> {
    let mut vec = s.clone();

    vec = vec[0..k].to_vec();

    vec.sort();

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["aaaaa", "abc", "xyz"], run(5, 3, vec!["abc", "aaaaa", "xyz", "a", "def"]));
        assert_eq!(vec!["rbg", "z", "zyx", "zzz"], run(4, 4, vec!["z", "zyx", "zzz", "rbg"]));
        assert_eq!(vec!["abc"], run(3, 1, vec!["abc", "arc", "agc"]));
    }
}
