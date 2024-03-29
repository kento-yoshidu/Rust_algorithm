// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_c

pub fn run(_n: usize, k: usize, pp: Vec<usize>, qq: Vec<usize>) -> String {
    for p in pp.iter() {
        for q in qq.iter() {
            if k == (p + q) {
                return String::from("Yes");
            }
        }
    }

    String::from("No")
}

pub fn run2(_n: usize, k: usize, pp: Vec<usize>, qq: Vec<usize>) -> String {
    if pp.iter().any(|p| {
        qq.iter().any(|q| {
            *p + *q == k
        })
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
        assert_eq!(String::from("No"), run(3, 100, vec![17, 57, 99], vec![10, 36, 53]));
        assert_eq!(String::from("Yes"), run(5, 53, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("No"), run2(3, 100, vec![17, 57, 99], vec![10, 36, 53]));
        assert_eq!(String::from("Yes"), run2(5, 53, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5]));
    }
}
