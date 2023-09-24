// https://atcoder.jp/contests/abc321/tasks/abc321_b

fn calc(n: isize, x: isize, vec: & Vec<isize>, i: isize) -> bool {
    let mut new_vec = vec.to_vec();

    new_vec.push(i);
    new_vec.sort();

    if new_vec[1..(n-1) as usize].iter().sum::<isize>() >= x {
        true
    } else {
        false
    }
}

pub fn run(n: isize, x: isize, a: Vec<isize>) -> isize {
    (0..101).find(|num| {
        calc(n, x, &a, *num)
    }).unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(70, run(5, 180, vec![40, 60, 80, 50]));
        assert_eq!(0, run(3, 100, vec![100, 100]));
        assert_eq!(-1, run(5, 200, vec![0, 0, 99, 99]));
        assert_eq!(45, run(10, 480, vec![59, 98, 88, 54, 70, 24, 8, 94, 46]));
    }
}
