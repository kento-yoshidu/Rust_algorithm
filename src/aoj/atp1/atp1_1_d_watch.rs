// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_D

#[allow(dead_code)]
pub fn run(t: i32) -> String {
    let mut time = t;

    let h = time / (60*60);

    time -= h*60*60;

    let m = time / 60;

    time -= m*60;

    format!("{}:{}:{}", h, m, time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("13:2:59"), run(46979));
    }
}
