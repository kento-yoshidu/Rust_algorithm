// https://atcoder.jp/contests/abc170/tasks/abc170_b

pub fn run(x: i32, y: i32) -> String {
    //　亀をi匹とする
    for i in 0..=x {
        let kame = i;
        let tsuru = x - i;

        if kame*4 + tsuru*2 == y {
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
        assert_eq!(String::from("Yes"), run(3, 8));
        assert_eq!(String::from("No"), run(2, 100));
        assert_eq!(String::from("Yes"), run(25, 100));
        assert_eq!(String::from("Yes"), run(1, 2));
    }
}
