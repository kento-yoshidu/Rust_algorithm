// https://atcoder.jp/contests/abc111/tasks/abc111_a

pub fn run(n: i32) -> i32 {
    let mut result = String::new();

    n.to_string().chars().for_each(|i| {
        if i == '9' {
            result.push('1')
        } else {
            result.push('9')
        }
    });

    result.parse::<i32>().unwrap()
}

fn run2(n: i32) -> i32 {
    let result = n.to_string().chars().map(|c| {
        match c {
            '1' => '9',
            _ => '1'
        }
    }).collect::<String>();

    result.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(991, run(119));
        assert_eq!(111, run(999));
    }

    #[test]
    fn test2() {
        assert_eq!(991, run2(119));
        assert_eq!(111, run2(999));
    }
}