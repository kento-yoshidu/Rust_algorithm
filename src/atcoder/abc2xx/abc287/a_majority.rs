// https://atcoder.jp/contests/abc287/tasks/abc287_a

pub fn run(_n: usize, vec: Vec<&str>) -> String {
    let mut f = 0;
    let mut a = 0;

    for v in vec.iter() {
        if *v == "For" {
            f += 1;
        } else {
            a += 1;
        }
    }

    if f > a {
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
        assert_eq!(String::from("Yes"), run(7, vec!["For", "Against", "For"]));
        assert_eq!(String::from("No"), run(5, vec!["Against", "Against", "For", "Against", "For"]));
        assert_eq!(String::from("Yes"), run(1, vec!["For"]));
    }
}
