// https://leetcode.com/problems/reverse-string/description/

// Memo: You must do this by modifying the input array in-place with O(1) extra memory.
fn run(s: &mut Vec<char>) {
    let mut left = 0;
    let mut right = s.len().saturating_sub(1);

    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<char>, Vec<char>);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec!['h','e','l','l','o'], vec!['o','l','l','e','h']),
            TestCase(vec!['H','a','n','n','a','h'], vec!['h','a','n','n','a','H']),
        ];

        for TestCase(mut s, expected) in tests {
            run(&mut s);
            assert_eq!(s, expected);
        }
    }
}