// https://atcoder.jp/contests/abc308/submissions/43327179

#[allow(dead_code)]
pub fn run(s: Vec<usize>) -> String {
    for i in s.iter() {
        if i % 25 != 0 || i < &100 || 675 < *i {
            return String::from("No");
        }
    }

    for i in 0..s.len()-1 {
        if s[i] >= s[i+1] {
            return String::from("No");
        }
    }

    String::from("Yes")
}

#[allow(dead_code)]
pub fn run2(s: Vec<usize>) -> String {
    let result = (0..7).all(|i| {
        s[i] <= s[i+1]
    }) &&
    s.iter().all(|i| {
        (i % 25 == 0) && (100 <= *i) && (*i <= 675)
    });

    if result == true {
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
        assert_eq!(String::from("Yes"), run(vec![125, 175, 250, 300, 400, 525, 600, 650]));
        assert_eq!(String::from("No"), run(vec![100, 250, 300, 400, 325, 575, 625, 675]));
        assert_eq!(String::from("No"), run(vec![0, 23, 24, 145, 301, 413, 631, 632]));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(vec![125, 175, 250, 300, 400, 525, 600, 650]));
        assert_eq!(String::from("No"), run2(vec![100, 250, 300, 400, 325, 575, 625, 675]));
        assert_eq!(String::from("No"), run2(vec![0, 23, 24, 145, 301, 413, 631, 632]));
    }
}
