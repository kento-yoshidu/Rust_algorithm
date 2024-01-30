// https://atcoder.jp/contests/abc205/tasks/abc205_c

pub fn run(a: i32, b: i32, c: i32) -> char {
    if c % 2 == 0 {
        if a.abs() == b.abs() {
            '='
        } else if a.abs() > b.abs() {
            '>'
        } else {
            '<'
        }
    } else {
        if a > b {
            '>'
        } else if a < b {
            '<'
        } else {
            '='
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('>', run(3, 2, 4));
        assert_eq!('=', run(-7, 7, 2));
        assert_eq!('<', run(-8, 3, 3));
        assert_eq!('<', run(796382932, -905246003, 182548924));
    }
}
