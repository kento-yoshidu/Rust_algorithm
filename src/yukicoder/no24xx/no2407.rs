// https://yukicoder.me/problems/no/2407

fn run(w: usize, z: usize, b: f64) -> usize {
    (w as f64 + z as f64 + w as f64 * b + z as f64 * b) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, f64, usize);

    #[test]
    fn yuki_2407() {
        let tests = [
            TestCase(100, 200, 2.0, 900),
            TestCase(150, 240, 1.6, 1014),
        ];

        for TestCase(w, z, b, expected) in tests {
            assert_eq!(run(w, z, b), expected);
        }
    }
}
