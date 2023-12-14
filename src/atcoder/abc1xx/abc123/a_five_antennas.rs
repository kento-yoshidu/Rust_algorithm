// https://atcoder.jp/contests/abc123/tasks/abc123_a

pub fn run(a: i32, b: i32, c: i32, d: i32, e: i32, k: i32) -> String {
    let vec = vec![a, b, c, d, e];

    let max = vec.iter().max().unwrap();
    let min = vec.iter().min().unwrap();

    if (max - min) <= k {
        String::from("Yay!")
    } else {
        String::from(":(")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yay!"), run(1, 2, 4, 8, 9, 15));
        assert_eq!(String::from(":("), run(15, 18, 26, 35, 36, 18));
    }
}
