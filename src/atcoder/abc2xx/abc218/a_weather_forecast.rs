// https://atcoder.jp/contests/abc218/tasks/abc218_a

pub fn run(n: usize, s: String) -> String {
    let tmp = s.chars().nth(n-1).unwrap();

    if tmp == 'o' {
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
        assert_eq!(String::from("No"), run(4, String::from("oooxoox")));
        assert_eq!(String::from("Yes"), run(7, String::from("ooooooo")));
    }
}
