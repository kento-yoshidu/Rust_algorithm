// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/all/ITP1_1_A

pub fn run() -> String {
    String::from("Hello World")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Hello World"), run());
    }
}
