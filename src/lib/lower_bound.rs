use std::cmp::Ordering;

pub fn lower_bound<T: Ord>(vec: &[T], value: T) -> usize {
    vec.binary_search_by(|x| {
        if *x < value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
    .err()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_bound_main() {
        let vec = vec![1, 3, 5, 7, 9];

        assert_eq!(lower_bound(&vec, -1), 0);
        assert_eq!(lower_bound(&vec, 0), 0);
        assert_eq!(lower_bound(&vec, 1), 0);
        assert_eq!(lower_bound(&vec, 2), 1);
        assert_eq!(lower_bound(&vec, 3), 1);
        assert_eq!(lower_bound(&vec, 4), 2);
        assert_eq!(lower_bound(&vec, 5), 2);
        assert_eq!(lower_bound(&vec, 6), 3);
        assert_eq!(lower_bound(&vec, 7), 3);
        assert_eq!(lower_bound(&vec, 8), 4);
        assert_eq!(lower_bound(&vec, 9), 4);
        assert_eq!(lower_bound(&vec, 10), 5);
    }

    #[test]
    fn lower_bound_duplicate() {
        // 要素が重複する配列を渡した時
        let vec = vec![1, 3, 3, 3, 5];

        assert_eq!(lower_bound(&vec, 0), 0);
        assert_eq!(lower_bound(&vec, 1), 0);
        assert_eq!(lower_bound(&vec, 2), 1);
        assert_eq!(lower_bound(&vec, 3), 1);
        assert_eq!(lower_bound(&vec, 4), 4);
        assert_eq!(lower_bound(&vec, 5), 4);
        assert_eq!(lower_bound(&vec, 6), 5);
    }

    #[test]
    fn lower_bound_empty_vec() {
        // 空配列を渡した時
        let vec: Vec<usize> = Vec::new();

        assert_eq!(lower_bound(&vec, 1), 0);
    }
}
