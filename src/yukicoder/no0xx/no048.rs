// https://yukicoder.me/problems/no/48

fn run(x: isize, y: isize, l: isize) -> isize {
    let x = x as f64;
    let y = y as f64;
    let l = l as f64;

    if x == 0.0 && y >= 0.0 {
        (y / l).ceil() as isize
    } else if x.abs() > 0.0 && y >= 0.0 {
        ((y / l).ceil() + 1.0 + (x.abs() / l).ceil()) as isize
    } else {
        (1.0 + (x.abs() / l).ceil() + 1.0 + (y.abs() / l).ceil()) as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase(isize, isize, isize, isize);

    #[test]
    fn yuki_048() {
        let tests = [
            TestCase(0, 2, 1, 2),
            TestCase(-7, 15, 7, 5),
            TestCase(1000000000, 1000000000, 999999997, 5),
        ];

        for TestCase(x, y, l, expected) in tests {
            assert_eq!(run(x, y, l), expected);
        }
    }
}
