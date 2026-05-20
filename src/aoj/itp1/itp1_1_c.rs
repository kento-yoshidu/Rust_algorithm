// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/1/ITP1_1_C

fn run(w: usize, h: usize) -> (usize, usize) {
    (w * h, (w + h) * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, (usize, usize));

    #[test]
    fn itp1_1_c() {
        assert_eq!((15, 16), run(3, 5));
    }
}
