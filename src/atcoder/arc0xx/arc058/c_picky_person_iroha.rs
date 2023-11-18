// https://atcoder.jp/contests/abc042/tasks/arc058_a

pub fn run(n: usize, _k: usize, d: Vec<usize>) -> usize {
    let chars: Vec<char> = d.iter()
        .map(|num| char::from_digit(*num as u32, 10).unwrap())
        .collect();

    (n..=n*10)
        .find(|num| {
            !num.to_string().contains(&*chars)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2000, run(1000, 8, vec![1, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(9999, run(9999, 1, vec![0]));
    }
}
