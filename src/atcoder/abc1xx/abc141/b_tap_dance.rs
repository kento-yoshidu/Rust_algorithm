// https://atcoder.jp/contests/abc141/tasks/abc141_b

pub fn run(s: &str) -> String {
    let ans = s.chars()
        .enumerate()
        .all(|(i, c)| {
            if c == 'U' || c == 'D' {
                true
            } else if i % 2 == 0 {
                c == 'R'
            } else {
                c == 'L'
            }
        });

    if ans == true {
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
        assert_eq!(String::from("Yes"), run("RUDLUDR"));
        assert_eq!(String::from("No"), run("DULL"));
        assert_eq!(String::from("Yes"), run("UUUUUUUUUUUUUUU"));
        assert_eq!(String::from("No"), run("ULURU"));
        assert_eq!(String::from("Yes"), run("RDULULDURURLRDULRLR"));
    }
}
