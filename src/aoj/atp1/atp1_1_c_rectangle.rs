// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/1/ITP1_1_C

pub fn run(w: i32, h: i32) -> (i32, i32) {
    (w*h, (w+h)*2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((15, 16), run(3, 5));
    }
}
