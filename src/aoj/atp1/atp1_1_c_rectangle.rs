#[allow(dead_code)]
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
