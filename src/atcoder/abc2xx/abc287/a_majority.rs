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

pub fn run2(_n: usize, vec: Vec<&str>) -> String {
    if 0 < vec.iter().map(|str| {
        if *str == "For" {
            1
        } else {
            -1
        }
    }).sum() {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

pub fn run3(_n: usize, vec: Vec<&str>) -> String {
    let f = vec.iter().filter(|str| **str == "For").count();
    let a = vec.iter().filter(|str| **str == "Against").count();

    if f > a {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

pub fn run4(_n: usize, vec: Vec<&str>) -> String {
    let (f, a) = vec.iter().fold((0, 0), |(f, a), s|
        match *s {
            "For" => (f + 1, a),
            "Against" => (f, a + 1),
            _ => unreachable!(),
        }
    );

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

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(7, vec!["For", "Against", "For"]));
        assert_eq!(String::from("No"), run2(5, vec!["Against", "Against", "For", "Against", "For"]));
        assert_eq!(String::from("Yes"), run2(1, vec!["For"]));
    }

    #[test]
    fn test3() {
        assert_eq!(String::from("Yes"), run3(7, vec!["For", "Against", "For"]));
        assert_eq!(String::from("No"), run3(5, vec!["Against", "Against", "For", "Against", "For"]));
        assert_eq!(String::from("Yes"), run3(1, vec!["For"]));
    }

    #[test]
    fn test4() {
        assert_eq!(String::from("Yes"), run4(7, vec!["For", "Against", "For"]));
        assert_eq!(String::from("No"), run4(5, vec!["Against", "Against", "For", "Against", "For"]));
        assert_eq!(String::from("Yes"), run4(1, vec!["For"]));
    }
}
