// https://atcoder.jp/contests/abc196/tasks/abc196_b

fn run(x: &str) -> String {
    if !x.contains(".") {
        return x.to_string()
    }

    let p = x.chars().position(|c| c == '.').unwrap();

    x[0..p].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("123"), run("123.456"));
        assert_eq!(String::from("0"), run("0"));
        assert_eq!(String::from("84939825309432908832902189"), run("84939825309432908832902189.9092309409809091329"));
    }
}
