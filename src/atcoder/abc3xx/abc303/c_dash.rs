// https://atcoder.jp/contests/abc303/tasks/abc303_c

pub fn run(_n: usize, _m: usize, h: isize, k: isize, s: &str, sy: Vec<(isize, isize)>) -> &'static str {
    let mut hp = h;

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

        if sy.contains(&pos) && hp < k {
            hp = k;
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
