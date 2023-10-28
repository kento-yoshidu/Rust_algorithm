// https://atcoder.jp/contests/abc205/tasks/abc205_c

pub fn run(a: i32, b: i32, _c: i32) -> char {
    if a.abs() == b.abs() {
        return '=';
    }

    if a < 0 {
        return '<';
    }

    if b < 0 {
        return '>';
    }

    if a.abs() > b.abs() {
        '>'
    } else {
        '<'
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
    }
}
