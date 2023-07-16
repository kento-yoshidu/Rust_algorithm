// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_b

#[allow(dead_code, unused)]
pub fn run(n: usize, x: usize, vec: Vec<usize>) -> String {
    for i in vec.iter() {
        if *i == x {
            return  String::from("Yes");
        }
    }
    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5, 40, vec![10, 20, 30, 40, 50]));
    }
}
