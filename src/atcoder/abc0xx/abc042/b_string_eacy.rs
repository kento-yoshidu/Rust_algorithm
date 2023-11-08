// https://atcoder.jp/contests/abc042/tasks/abc042_b

pub fn run(_n: usize, _l: usize, s: Vec<&str>) -> String {
    let mut vec = s.clone();
    vec.sort();

    vec.iter()
        .map(|str| str.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("axxcxxdxx"), run(3, 3, vec!["dxx", "axx", "cxx"]));
    }
}
