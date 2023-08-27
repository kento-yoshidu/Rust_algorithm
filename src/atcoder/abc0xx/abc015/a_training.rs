// https://atcoder.jp/contests/abc015/tasks/abc015_1

pub fn run(a: &str, b: &str) -> String {
    if a.len() > b.len() {
        a.to_string()
    } else {
        b.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("isleapyear"), run("isuruu", "isleapyear"));
        assert_eq!(String::from("ttttiiiimmmmeeee"), run("ttttiiiimmmmeeee", "time"));
    }
}
