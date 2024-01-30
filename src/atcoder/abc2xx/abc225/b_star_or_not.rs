// https://atcoder.jp/contests/abc225/tasks/abc225_b

pub fn run(n: usize, ab: Vec<(usize, usize)>) -> String {
    let count = ab.iter()
        .fold(vec![0; n], |mut state, (a, b)| {
            state[*a-1] += 1;
            state[*b-1] += 1;
            state
        });

    if count.iter()
        .any(|num| {
            *num == n - 1
        }) {
            String::from("Yes")
        } else {
            String::from("No")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5, vec![(1, 4), (2, 4), (3, 4), (4, 5)]));
        assert_eq!(String::from("No"), run(4, vec![(2, 4), (1, 4), (2, 3)]));
        assert_eq!(String::from("Yes"), run(10, vec![(9, 10), (3, 10), (4, 10), (8, 10), (1, 10), (2, 10), (7, 10), (6, 10), (5, 10)]));
    }
}
