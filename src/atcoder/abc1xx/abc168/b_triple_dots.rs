// https://atcoder.jp/contests/abc168/tasks/abc168_b

pub fn run(k: usize, s: &str) -> String {
    if k >= s.len() {
        s.to_string()
    } else {
        format!("{}...", &s[0..k])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("nikoand..."), run(7, "nikoandsolstice"));
        assert_eq!(String::from("ferelibenterhominesidquodvoluntcredunt"), run(40, "ferelibenterhominesidquodvoluntcredunt"));
    }
}
