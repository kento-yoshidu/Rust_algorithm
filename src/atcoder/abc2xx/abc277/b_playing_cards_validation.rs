// https://atcoder.jp/contests/abc277/tasks/abc277_b

use itertools::Itertools;

pub fn run(_n: usize, s: Vec<&str>) -> String {
    if !s.iter().all_unique() {
        return String::from("No")
    }

    let vec: Vec<Vec<char>> =
        s.iter()
            .map(|str| str.chars().collect::<Vec<char>>())
            .collect();

    if vec.iter()
        .all(|v| {
            ['H', 'D', 'C', 'S'].contains(&v[0])
            &&
            ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'].contains(&v[1])
        }) {
            String::from("Yes")
        } else {
            String::from("No")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(4, vec!["H3", "DA", "D3", "SK"]));
        assert_eq!(String::from("No"), run(5, vec!["H3", "DA", "CK", "H3", "S7"]));
        assert_eq!(String::from("No"), run(4, vec!["3H", "AD", "3D", "KS"]));
        assert_eq!(String::from("No"), run(5, vec!["00", "AA", "XX", "YY", "ZZ"]));
    }
}
