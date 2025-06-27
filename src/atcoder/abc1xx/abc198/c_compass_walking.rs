// https://atcoder.jp/contests/abc198/tasks/abc198_c

fn run(r: usize, x: usize, y: usize) -> usize {
    let r = r as f64;
    let x = x as f64;
    let y = y as f64;

    let dist = (x * x + y * y).sqrt();

    if dist == r {
        1
    } else if dist <= r + r {
        2
    } else {
        (dist / r).ceil() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 15, 0, 3),
            TestCase(5, 11, 0, 3),
            TestCase(3, 4, 4, 2),
        ];

        for TestCase(r, x, y, expected) in tests {
            assert_eq!(run(r, x, y), expected);
        }
    }
}
