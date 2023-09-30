// https://atcoder.jp/contests/abc073/tasks/abc073_a

pub fn run(n: usize) -> String {
	if n >= 90 || n % 10 == 9 {
		String::from("Yes")
	} else {
		String::from("No")
	}
}

pub fn run2(n: usize) -> String {
	if n.to_string().chars().any(|c| {
		c == '9'
	}) {
		String::from("Yes")
	} else {
		String::from("No")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Yes"), run(29));
		assert_eq!(String::from("No"), run(72));
		assert_eq!(String::from("Yes"), run(91));
	}

	#[test]
	fn test2() {
		assert_eq!(String::from("Yes"), run2(29));
		assert_eq!(String::from("No"), run2(72));
		assert_eq!(String::from("Yes"), run2(91));
	}
}
