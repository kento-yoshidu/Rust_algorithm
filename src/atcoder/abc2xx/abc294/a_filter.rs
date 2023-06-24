/*
pub fn run(vec: Vec<i32>) -> Vec<&'static i32> {
    let result = vec.clone().iter().filter(|&i| i % 2 == 0).collect::<Vec<&i32>>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2, 6], run(vec![1, 2, 3, 5, 6]))
    }
}
*/
