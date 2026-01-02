// https://atcoder.jp/contests/abc308/tasks/abc308_a
// https://atcoder.jp/contests/abc308/submissions/43327179

fn run(s: &Vec<usize>) -> &'static str {
    for i in s.iter() {
        if i % 25 != 0 || i < &100 || 675 < *i {
            return "No";
        }
    }

    for i in 0..s.len()-1 {
        if s[i] >= s[i+1] {
            return "No";
        }
    }

    "Yes"
}

fn run2(s: &Vec<usize>) -> &'static str {
    let result = (0..7).all(|i| {
        s[i] <= s[i+1]
    }) &&
    s.iter().all(|i| {
        (i % 25 == 0) && (100 <= *i) && (*i <= 675)
    });

    if result {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, &'static str);

    #[test]
    fn abc308_a() {
        let tests = [
            TestCase(vec![125, 175, 250, 300, 400, 525, 600, 650], "Yes"),
            TestCase(vec![100, 250, 300, 400, 325, 575, 625, 675], "No"),
            TestCase(vec![0, 23, 24, 145, 301, 413, 631, 632], "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(&s), expected);
            assert_eq!(run2(&s), expected);
        }
    }
}
