#[allow(dead_code)]
pub fn run(n: i32, vec: Vec<i32>) -> i32 {
    let mut height = vec[0];

    let mut ans = 0;

    for h in vec.iter() {
        if height <= *h {
            ans += 1;
            height = *h;
        }
    }

    ans
}

/*
pub fn run2(n: i32, vec: Vec<i32>) -> usize {
    let mut height = vec[0];

    let ans = vec.iter().filter(|&h| {
        if height
        height <= *h;
    }).count();

    println!("{:?}", ans);

    10
}

    let ans: usize = vec.iter().map(|&h| {
        height <= h;
        height = height.max(h);
    }).count();
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, vec![6, 5, 6, 8]));
        assert_eq!(3, run(5, vec![4, 5, 3, 5, 4]));
        assert_eq!(1, run(5, vec![9, 5, 6, 8, 4]));
        assert_eq!(4, run(5, vec![1, 2, 3, 9, 4]));
    }
}
