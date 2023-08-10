// https://atcoder.jp/contests/abc313/tasks/abc313_b

fn run(n: usize, _m: usize, vec: Vec<(usize, usize)>) -> i32 {
    let mut arr = vec![true; n];

    for v in vec.iter() {
        arr[(v.1)-1] = false
    }

    if (arr.iter().filter(|num| {
        **num == true
    }).count()) > 1 {
        return -1
    } else {
        arr.iter().position(|num| *num == true).unwrap() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, 2, vec![(1, 2), (2, 3)]));
        assert_eq!(-1, run(3, 2, vec![(1, 3), (2, 3)]));
        assert_eq!(-1, run(6, 6, vec![(1, 6), (6, 5), (6, 2), (2, 3), (4, 3), (4, 2)]));
    }
}
