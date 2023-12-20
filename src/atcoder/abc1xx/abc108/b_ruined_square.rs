// https://atcoder.jp/contests/abc108/tasks/abc108_b

pub fn run(p1: (i32, i32), p2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dy = p2.0 - p1.0;
    let dx = p2.1 - p1.1;

    let p3 = (p2.0 - dx, p2.1 + dy);
    let p4 = (p1.0 - dx, p1.1 + dy);

    (p3, p4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(((-1, 1), (-1, 0)), run((0, 0), (0, 1)));
        assert_eq!(((3, 10), (-1, 7)), run((2, 3), (6, 6)));
        assert_eq!(((-126, -64), (-36, -131)), run((31, -41),  (-59, 26)));
    }
}
