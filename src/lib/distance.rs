pub fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn is_within_euclidean_distance(x1: isize, y1: isize, x2: isize, y2: isize, r: usize) -> bool {
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);

    dx * dx + dy * dy <= r * r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_distance_main() {
        assert_eq!(manhattan_distance(0, 0, 3, 4), 7);
        assert_eq!(manhattan_distance(-1, -1, 2, 3), 7);
        assert_eq!(manhattan_distance(5, 5, 5, 5), 0);
    }

    #[test]
    fn is_within_euclidean_distance_main() {
        assert_eq!(is_within_euclidean_distance(0, 0, 3, 4, 5), true);
        assert_eq!(is_within_euclidean_distance(0, 0, 6, 0, 5), false);
        assert_eq!(is_within_euclidean_distance(0, 0, -1, -1, 5), true);
    }
}
