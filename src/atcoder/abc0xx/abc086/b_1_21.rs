// https://atcoder.jp/contests/abc086/tasks/abc086_b

use num_integer::Roots;

#[allow(dead_code)]
pub fn run(a: String, b: String) -> String {
    let num: i32 = (a + b.as_str()).parse().unwrap();

    if num.sqrt().pow(2) == num {
        String::from("Yes")
    }
    else {
        String::from("No")
    }
}

#[allow(dead_code)]
pub fn run2(a: String, b: String) -> String {
    let num: i32 = (a + b.as_str()).parse().unwrap();

    for i in 1..=(num.sqrt()) {
        if i * i == num {
            return String::from("Yes");
        }
    }

    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(String::from("1"), String::from("21")));
        assert_eq!(String::from("Yes"), run(String::from("1"), String::from("44")));
        assert_eq!(String::from("Yes"), run(String::from("1"), String::from("69")));
        assert_eq!(String::from("No"), run(String::from("100"), String::from("100")));
        assert_eq!(String::from("No"), run(String::from("12"), String::from("10")));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(String::from("1"), String::from("21")));
        assert_eq!(String::from("Yes"), run2(String::from("1"), String::from("44")));
        assert_eq!(String::from("Yes"), run2(String::from("1"), String::from("69")));
        assert_eq!(String::from("No"), run2(String::from("12"), String::from("10")));
    }
}
