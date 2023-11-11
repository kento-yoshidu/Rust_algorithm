// https://atcoder.jp/contests/abc089/tasks/abc089_b

pub fn run(_n: usize, s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();
    chars.dedup();

    match chars.len() {
        3 => String::from("Three"),
        4 => String::from("Four"),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Four"), run(6, String::from("GWYPYW")));
        assert_eq!(String::from("Three"), run(9, String::from("GWWGPWPGG")));
        assert_eq!(String::from("Four"), run(8, String::from("PYWGYWYY")));
    }
}