// https://atcoder.jp/contests/abc176/tasks/abc176_b

pub fn run(n: &str) -> String {
    let total: usize = n.chars()
        .map(|c| {
            c as usize - 48
        })
        .sum();

    if total % 9 == 0 {
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
        assert_eq!(String::from("Yes"), run("123456789"));
        assert_eq!(String::from("Yes"), run("0"));
        assert_eq!(String::from("No"), run("31415926535897932384626433832795028841971693993751058209749445923078164062862089986280"));
    }
}
