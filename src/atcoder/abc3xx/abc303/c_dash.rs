// https://atcoder.jp/contests/abc303/tasks/abc303_c

use std::collections::HashSet;

pub fn run(_n: usize, _m: usize, h: isize, k: isize, s: &str, xy: Vec<(isize, isize)>) -> &'static str {
    let mut hp = h;

    let mut vec = HashSet::new();

    for i in xy {
        vec.insert(i);
    }

    let mut pos: (isize, isize) = (0, 0);

    for c in s.chars() {
        match c {
            'R' => pos.0 += 1,
            'L' => pos.0 -= 1,
            'U' => pos.1 += 1,
            'D' => pos.1 -= 1,
            _ => unreachable!(),
        }

        hp -= 1;

        if hp < 0 {
            return "No"
        }

        if vec.contains(&pos) && hp < k {
            hp = k;
            vec.remove(&pos);
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run(4, 2, 3, 1, "RUDL", vec![(-1, -1), (1, 0)]));
        assert_eq!("No", run(5, 2, 1, 5, "LDRLD", vec![(0, 0), (-1, -1)]));
    }
}
