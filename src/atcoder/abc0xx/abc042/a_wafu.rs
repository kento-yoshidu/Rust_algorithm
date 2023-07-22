// https://atcoder.jp/contests/abc042/tasks/abc042_a

#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> String {
    let vec = vec![a, b, c];

    let mut five = 0;
    let mut seven = 0;

    for i in vec.iter() {
        match i {
            5 => five += 1,
            7 => seven += 1,
            _ => continue
        }
    }

    if five == 2 && seven == 1 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("YES"), run(5, 5, 7));
        assert_eq!(String::from("NO"), run(7, 7, 5));
    }
}
