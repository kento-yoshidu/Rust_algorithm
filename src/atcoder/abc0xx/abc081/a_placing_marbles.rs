// https://atcoder.jp/contests/abc081/tasks/abc081_a

pub fn run(str: &str) -> i32 {
    let mut count = 0;

    for c in str.chars() {
        if c == '1' {
            count += 1;
        }
    }

    count
}

pub fn run2(str: String) -> usize {
    str.chars().filter(|c| {
        *c == '1'
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run("101"));
        assert_eq!(0, run("000"));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(String::from("101")));
        assert_eq!(0, run2(String::from("000")));
    }
}