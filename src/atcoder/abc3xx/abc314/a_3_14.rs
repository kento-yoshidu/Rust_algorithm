// https://atcoder.jp/contests/abc314/tasks/abc314_a

pub fn run(n: usize) -> String {
    let pi: Vec<char> = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".chars().collect();

    pi[0..n+2].iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("3.14"), run(2));
        assert_eq!(String::from("3.14159265358979323846264338327950"), run(32));
        assert_eq!(String::from("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679"), run(100));
    }
}
