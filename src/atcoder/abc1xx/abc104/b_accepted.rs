pub fn run(s: String) -> String {
    let c: Vec<char> = s.chars().collect();

    if c[0] != 'A' {
        return String::from("WA");
    }

    let c_count = c[2..c.len()-1].iter().filter(|&c| {
        *c == 'C'
    }).count();

    if c_count != 1 {
        return String::from("WA");
    }

    let upper_count = c.iter().filter(|&c| {
        c.is_uppercase()
    }).count();

    if upper_count != 2 {
        return String::from("WA");
    }

    return String::from("AC")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("AC"), run(String::from("AtCoder")));
        assert_eq!(String::from("WA"), run(String::from("ACoder")));
        assert_eq!(String::from("WA"), run(String::from("AcycliC")));
        assert_eq!(String::from("WA"), run(String::from("AtCoCo")));
        assert_eq!(String::from("WA"), run(String::from("Atcoder")));
    }
}
