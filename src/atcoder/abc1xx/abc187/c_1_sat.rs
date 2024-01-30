// https://atcoder.jp/contests/abc187/tasks/abc187_c

pub fn run(_n: usize, a: Vec<&str>) -> String {
    let (mut a, mut b): (Vec::<String>, Vec::<String>) = a.iter()
        .map(|t| {
            t.to_string()
        })
        .partition(|c| {
            c.contains("!")
        });

    a.sort();
    a.dedup();

    b.sort();
    b.dedup();

    b.iter()
        .find(|str| {
            a.contains(&("!".to_owned() + str))
        })
        .unwrap_or(&"satisfiable".to_string())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("a"), run(6, vec!["a", "!a", "b", "!c", "d", "!d"]));
        assert_eq!(String::from("satisfiable"), run(10, vec!["red", "red", "red", "!orange", "yellow", "!blue", "cyan", "!green", "brown", "!gray"]));
        assert_eq!(String::from("e"), run(22, vec!["!u", "!g", "!f", "!a", "!u", "!x", "q", "z", "!o", "!b", "!j", "!h", "e", "!i", "s", "!e", "!x", "l", "n", "j", "!c", "m"]));
    }
}
