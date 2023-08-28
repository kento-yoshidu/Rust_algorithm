// https://atcoder.jp/contests/arc105/tasks/arc105_a

fn run(a: usize, b: usize, c: usize, d: usize) -> String {
    let vec = vec![a, b, c, d];

    for bit in 0..(1 << 4) {
        let mut eat = 0;
        let mut rest = 0;

        for i in 0..4 {
            if bit & (1 << i) != 0 {
                eat += vec[i];
            } else {
                rest += vec[i];
            }
        }

        if eat == rest {
            return String::from("Yes");
        }
    }

    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(1, 3, 2, 4));
        assert_eq!(String::from("No"), run(1, 2, 4, 8));
        assert_eq!(String::from("Yes"), run(1, 1, 1, 1));
        assert_eq!(String::from("Yes"), run(1, 100, 50, 51));
        assert_eq!(String::from("Yes"), run(2, 100, 48, 50));
        assert_eq!(String::from("Yes"), run(63214004, 4741111, 4654151, 63300964));
        assert_eq!(String::from("No"), run(4630987, 9157337, 18793476, 5005153));
        assert_eq!(String::from("No"), run(93407609, 30427494, 56229544, 81174201));
    }
}
