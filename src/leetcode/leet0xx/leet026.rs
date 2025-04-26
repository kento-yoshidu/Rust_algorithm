// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/

fn run(nums: Vec<i32>) -> usize {
    let mut nums = nums.clone();

    nums.dedup();

    nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<i32>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![1, 1, 2], 2),
            TestCase(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5),
        ];

        for TestCase(nums, expected) in tests {
            assert_eq!(run(nums), expected);
        }
    }
}
