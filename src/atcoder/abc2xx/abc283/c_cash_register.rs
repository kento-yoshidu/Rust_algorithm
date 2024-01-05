// https://atcoder.jp/contests/abc283/tasks/abc283_c

pub fn run(s: &str) -> usize {
    let mut vec: Vec<char> = s.chars().collect();

    let mut count = 0;

    for i in 0..(vec.len()-1) {
        if vec[i] == '0' && vec[i+1] == '0' {
            vec[i+1] = 'x';
            count += 1;
        }
    }

    vec.len() - count
}

pub fn run2(s: &str) -> usize {
    let mut vec: String = s.chars().collect();

    vec = vec.replace("00", "x");

    vec.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run("40004"));
        assert_eq!(10, run("1355506027"));
        assert_eq!(27, run("10888869450418352160768000001"));
    }

    #[test]
    fn test2() {
        assert_eq!(4, run2("40004"));
        assert_eq!(10, run2("1355506027"));
        assert_eq!(27, run2("10888869450418352160768000001"));
    }
}
