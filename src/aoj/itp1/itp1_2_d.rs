// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/2/ITP1_2_D

fn run(w: isize, h: isize, x: isize, y: isize, r: isize) -> &'static str {
    if x - r >= 0 && x + r <= w && y - r >= 0 && y + r <= h {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize, &'static str);

    #[test]
    fn itp1_2_d() {
        let tests = [
            TestCase(5, 4, 2, 2, 1, "Yes"),
            TestCase(5, 4, 2, 4, 1, "No"),
        ];

        for TestCase(w, h, x, y, r, expected) in tests {
            assert_eq!(run(w, h, x, y, r), expected);
        }
    }
}
