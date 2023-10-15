// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_b

pub fn run(_n: usize, x: usize, vec: Vec<usize>) -> String {
    for i in vec.iter() {
        if *i == x {
            return  String::from("Yes");
        }
    }
    String::from("No")
}

pub fn run2(_n: usize, x: usize, vec: Vec<usize>) -> String {
    if vec.iter().any(|num| {
        *num == x
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
        assert_eq!(String::from("Yes"), run(5, 40, vec![10, 20, 30, 40, 50]));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(5, 40, vec![10, 20, 30, 40, 50]));
    }
}
