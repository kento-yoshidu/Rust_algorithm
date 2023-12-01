// https://atcoder.jp/contests/abc296/tasks/abc296_b

pub fn run(s: Vec<&str>) -> String {
    let vec: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();

    for i in 0..8 {
        for j in 0..8 {
            if vec[i][j] == '*' {
                return format!("{}{}", (b'a' + j as u8)as char, 8 - i);
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("a1"), run(vec!["........", "........", "........", "........", "........", "........", "........", "*......."]));
        assert_eq!(String::from("b3"), run(vec!["........", "........", "........", "........", "........", ".*......", "........", "........"]));
    }
}
