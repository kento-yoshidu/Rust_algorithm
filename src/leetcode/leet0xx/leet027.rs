// https://leetcode.com/problems/remove-element/description/

// Memo: remove all occurrences of val in nums in-place.
fn run(nums: Vec<usize>, val: usize) -> usize {
    let mut nums = nums.clone();

    let mut k = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    k
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![3, 2, 2, 3], 3, 2),
            TestCase(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5),
        ];

        for TestCase(nums, val, expected) in tests {
            assert_eq!(run(nums, val), expected);
        }
    }
}
