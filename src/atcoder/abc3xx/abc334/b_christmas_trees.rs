// https://atcoder.jp/contests/abc334/tasks/abc334_b

fn run(a: isize, m: isize, l: isize, r: isize) -> usize {
    let left = l - a;
    let right = r - a;

    let kl = if left >= 0 {
        (left + m - 1) / m
    } else {
        left / m
    };

    let kr = if right >= 0 {
        right / m
    } else {
        (right - m + 1) / m
    };

    if kr >= kl {
        (kr - kl + 1) as usize
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, usize);

    #[test]
    fn abc334_b() {
        let tests = [
            TestCase(5, 3, -1, 6, 3),
            TestCase(-2, 2, 1, 1, 0),
            TestCase(-177018739841739480, 2436426, -80154573737296504, 585335723211047198, 273142010859),
        ];

        for TestCase(a, m, l, r, expected) in tests {
            assert_eq!(run(a, m, l, r), expected);
        }
    }
}
