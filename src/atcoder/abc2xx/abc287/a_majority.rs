// https://atcoder.jp/contests/abc287/tasks/abc287_a

fn run(_n: usize, s: &Vec<&str>) -> &'static str {
    let mut f = 0;
    let mut a = 0;

    for v in s.iter() {
        if *v == "For" {
            f += 1;
        } else {
            a += 1;
        }
    }

    if f > a {
        "Yes"
    } else {
        "No"
    }
}

fn run2(_n: usize, s: &Vec<&str>) -> &'static str {
    if 0 < s.iter().map(|str| {
        if *str == "For" {
            1
        } else {
            -1
        }
    }).sum() {
        "Yes"
    } else {
        "No"
    }
}

fn run3(_n: usize, s: &Vec<&str>) -> &'static str {
    let f = s.iter().filter(|str| **str == "For").count();
    let a = s.iter().filter(|str| **str == "Against").count();

    if f > a {
        "Yes"
    } else {
        "No"
    }
}

fn run4(_n: usize, s: &Vec<&str>) -> &'static str {
    let (f, a) = s.iter().fold((0, 0), |(f, a), s|
        match *s {
            "For" => (f + 1, a),
            "Against" => (f, a + 1),
            _ => unreachable!(),
        }
    );

    if f > a {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, vec!["For", "Against", "For"], "Yes"),
            TestCase(5, vec!["Against", "Against", "For", "Against", "For"], "No"),
            TestCase(1, vec!["For"], "Yes"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, &s), expected);
            assert_eq!(run2(n, &s), expected);
            assert_eq!(run3(n, &s), expected);
            assert_eq!(run4(n, &s), expected);
        }
    }
}
