// https://atcoder.jp/contests/abc266/tasks/abc266_c

fn cross(ax: isize, ay: isize, bx: isize, by: isize) -> isize {
    ax * by - ay * bx
}

fn is_convex(points: [(isize, isize); 4]) -> bool {
    let mut signs = vec![];

    for i in 0..4 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % 4];
        let (x2, y2) = points[(i + 2) % 4];

        let ab = (x1 - x0, y1 - y0);
        let bc = (x2 - x1, y2 - y1);

        let z = cross(ab.0, ab.1, bc.0, bc.1);
        signs.push(z.signum());
    }

    signs.iter().all(|&s| s > 0) || signs.iter().all(|&s| s < 0)
}


fn run(a: (isize, isize), b: (isize, isize), c: (isize, isize), d: (isize, isize)) -> &'static str {
    if is_convex([a, b, c, d]) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase((isize, isize), (isize, isize), (isize, isize), (isize, isize), &'static str);

    #[test]
    fn abc266_c() {
        let tests = [
            TestCase((0, 0), (1, 0), (1, 1), (0, 1), "Yes"),
            TestCase((0, 0), (1, 1), (-1, 0), (1, -1), "No"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
