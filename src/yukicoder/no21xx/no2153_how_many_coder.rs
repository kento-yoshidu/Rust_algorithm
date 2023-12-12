use std::collections::HashMap;

pub fn run(_n: usize, v: Vec<(&str, usize)>) -> Vec<usize> {
    let mut hashmap = HashMap::new();

    v.iter()
        .for_each(|t| {
            hashmap.insert(t.0, t.1);
        });

    (0..=7)
        .map(|i| {
            hashmap.iter()
                .filter(|t| {
                    *t.1 == i
                })
                .count()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1, 0, 1, 0, 1, 0, 0], run(3, vec![("hibit", 3), ("KowerKoint", 5), ("Michirakara", 1)]));
        assert_eq!(vec![0, 0, 2, 0, 0, 0, 0, 0], run(4, vec![("Michirakara", 1), ("Michirakara", 2), ("hibit", 3), ("hibit", 2)]));
    }
}
