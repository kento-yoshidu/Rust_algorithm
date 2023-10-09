// https://atcoder.jp/contests/abc298/tasks/abc298_a

pub fn run(_n: usize, str: String) -> String {
    let ok = str.contains('o');
    let reject = str.contains('x');

    if ok && !reject {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run(4, "oo--".to_string()));
        assert_eq!("No", run(3, "---".to_string()));
        assert_eq!("Yes", run(1, "o".to_string()));
        assert_eq!("No", run(100, "ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooox".to_string()));
    }
}
