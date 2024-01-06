// https://atcoder.jp/contests/abc281/tasks/abc281_b

pub fn run(s: &str) -> &'static str {
    if s.len() < 8 {
        return "No"
    }

    if !s.chars().nth(0).unwrap().is_uppercase() || !s.chars().last().unwrap().is_uppercase() {
        return "No"
    }

    let str = &s[1..s.len()-1];

    if str.chars().nth(0).unwrap() == '0' {
        return "No"
    }

    if let Ok(num) = str.parse::<usize>() {
        if 111111 <= num && num <= 999999 {
            return "Yes";
        } else {
            return "No";
        }
    } else {
        return "No";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run("Q142857Z"));
        assert_eq!("No", run("AB912278C"));
        assert_eq!("No", run("X900000"));
        assert_eq!("No", run("K012345K"));
        assert_eq!("No", run("P0123456J"));
    }
}
