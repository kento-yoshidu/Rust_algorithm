// https://atcoder.jp/contests/abc279/tasks/abc279_a

pub fn run(str: &str) -> usize {
    let mut count = 0;

    for c in str.chars() {
        if c == 'w' {
            count = count + 2;
        } else {
            count = count + 1;
        }
    }

    count
}

pub fn run2(str: String) -> usize {
    str.chars().map(|c| {
        if c == 'w' {
            2
        } else {
            1
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, run("vvwvw"));
        assert_eq!(1, run("v"));
        assert_eq!(12, run("wwwvvvvvv"));
        assert_eq!(155, run("wwvvvwwwwvvwwwwwwwvwvvvwwvvvvvwwwwvwwvwvvwwwvvvwvvwvwwwvwwvvwvwwvvvvwvwwvvwwwvvwwwvwvwwwwwwvwvwvvvwv"));
        assert_eq!(147, run("wwwwvvwvvwwvvvvwvvvwvvwvvwwwvwvvwvvwwwvvvvwvvwwvwvwvwwvvwwvvvvvvwvwwwwvwwvwwvvwvvvwvvwwwwvvwvwwvvwvw"));
        assert_eq!(144, run("vwwvvwvvvwwvvwvwvwwvvwvvvvwvwvwvwvwvwwvvvvwvwwwvwwvwvwwwvwvvvvwvwvwwvvvwwvvvwvwvwwvvvvwvvvwvwwwvvwvv"));
        assert_eq!(32, run("vwvwvvvvwvvwwvwwwwvvvw"));
        assert_eq!(147, run("wwwwvvwvvwwvvvvwvvvwvvwvvwwwvwvvwvvwwwvvvvwvvwwvwvwvwwvvwwvvvvvvwvwwwwvwwvwwvvwvvvwvvwwwwvvwvwwvvwvw"));
        assert_eq!(29, run("wwvvvvwvwvwvwvvvwvvwv"));
        assert_eq!(16, run("vwvwvwwwwv"));
    }

    #[test]
    fn test2() {
        assert_eq!(7, run2(String::from("vvwvw")));
        assert_eq!(1, run2(String::from("v")));
        assert_eq!(12, run2(String::from("wwwvvvvvv")));
        assert_eq!(155, run2(String::from("wwvvvwwwwvvwwwwwwwvwvvvwwvvvvvwwwwvwwvwvvwwwvvvwvvwvwwwvwwvvwvwwvvvvwvwwvvwwwvvwwwvwvwwwwwwvwvwvvvwv")));
        assert_eq!(147, run2(String::from("wwwwvvwvvwwvvvvwvvvwvvwvvwwwvwvvwvvwwwvvvvwvvwwvwvwvwwvvwwvvvvvvwvwwwwvwwvwwvvwvvvwvvwwwwvvwvwwvvwvw")));
        assert_eq!(144, run2(String::from("vwwvvwvvvwwvvwvwvwwvvwvvvvwvwvwvwvwvwwvvvvwvwwwvwwvwvwwwvwvvvvwvwvwwvvvwwvvvwvwvwwvvvvwvvvwvwwwvvwvv")));
        assert_eq!(32, run2(String::from("vwvwvvvvwvvwwvwwwwvvvw")));
        assert_eq!(147, run2(String::from("wwwwvvwvvwwvvvvwvvvwvvwvvwwwvwvvwvvwwwvvvvwvvwwvwvwvwwvvwwvvvvvvwvwwwwvwwvwwvvwvvvwvvwwwwvvwvwwvvwvw")));
        assert_eq!(29, run2(String::from("wwvvvvwvwvwvwvvvwvvwv")));
        assert_eq!(16, run2(String::from("vwvwvwwwwv")));
    }
}
