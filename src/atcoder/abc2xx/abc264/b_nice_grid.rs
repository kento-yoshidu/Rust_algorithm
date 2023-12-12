// https://atcoder.jp/contests/abc264/tasks/abc264_b

use std::cmp::max;

pub fn run(r: i32, c: i32) -> String {
    let dis = max((r-8).abs(), (c-8).abs());

    if dis % 2 == 0 {
        String::from("white")
    } else {
        String::from("black")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("black"), run(3, 5));
        assert_eq!(String::from("white"), run(4, 5));
    }
}
