pub fn run_length<T>(s: Vec<T>) -> Vec<(T, usize)>
where
    T: PartialEq + Clone
{
    let mut result = vec![];
    let mut current = (s[0].clone(), 1);

    for i in 1..s.len() {
        if s[i] == current.0 {
            current.1 += 1;
        } else {
            result.push(current);
            current = (s[i].clone(), 1);
        }
    }

    result.push(current);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_length_main() {
        let vec = vec!['a', 'a', 'b', 'b', 'b', 'c', 'c', 'c', 'c'];
        assert_eq!(run_length(vec), vec![('a', 2), ('b', 3), ('c', 4)]);

        let vec = vec![1, 1, 2, 2, 2, 3, 3, 3];
        assert_eq!(run_length(vec), vec![(1, 2), (2, 3), (3, 3)]);

        let vec = vec!["aaa", "aaa", "bbb", "bbb", "bbb", "ccc", "ccc", "ccc"];
        assert_eq!(run_length(vec), vec![("aaa", 2), ("bbb", 3), ("ccc", 3)]);

        let vec = vec!['a', 'a', 'b', 'b', 'a', 'a'];
        assert_eq!(run_length(vec), vec![('a', 2), ('b', 2), ('a', 2)]);

        // 全て同じ文字の場合
        let vec = vec![1, 1, 1, 1];
        assert_eq!(run_length(vec), vec![(1, 4)]);
    }
}
