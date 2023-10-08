// https://atcoder.jp/contests/abc109/tasks/abc109_a

pub fn run(a: usize, b: usize) -> String {
	if a * b % 2 == 0 {
		String::from("No")
	} else {
		String::from("Yes")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Yes"), run(3, 1));
		assert_eq!(String::from("No"), run(1, 2));
		assert_eq!(String::from("No"), run(2, 2));
	}
}
