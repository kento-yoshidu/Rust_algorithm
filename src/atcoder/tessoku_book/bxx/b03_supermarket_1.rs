// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cb

#[allow(dead_code)]
pub fn run(n: usize, vec: Vec<usize>) -> String {
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                println!("i={}, j={}, k={}", vec[i], vec[j], vec[k]);

                if vec[i] + vec[j] + vec[k] == 1000 {
                    return  String::from("Yes");
                }
            }
        }
    }

    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5, vec![100, 250, 350, 400, 600]));
        assert_eq!(String::from("No"), run(10, vec![50, 150, 250, 350, 450, 550, 650, 750, 850, 950]))

    }
}
