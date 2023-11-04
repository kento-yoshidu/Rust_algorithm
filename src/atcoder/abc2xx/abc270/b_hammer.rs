// https://atcoder.jp/contests/abc270/tasks/abc270_b

pub fn run(x: isize, y: isize, z: isize) -> isize {
    // 壁 - 目標 - 武器
    if y < x && x < z {
        -1
    // 壁にぶつからずに行ける
    } else if y < 0 || x < y {
        x
    } else {
        x + (y - z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(10, -10, 1));
        assert_eq!(40, run(20, 10, -10));
        assert_eq!(-1, run(100, 1, 1000));
    }
}
