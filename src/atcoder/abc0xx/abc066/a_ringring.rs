// https://atcoder.jp/contests/abc066/tasks/abc066_a

pub fn run(a: i32, b: i32, c: i32) -> i32 {
    let mut vec = vec![a, b, c];

    vec.sort();

    vec.iter().nth(0).unwrap() + vec.iter().nth(1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1300, run(700, 600, 780));
        assert_eq!(20000, run(10000, 10000, 10000));
    }
}
