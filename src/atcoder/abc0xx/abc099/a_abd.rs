// https://atcoder.jp/contests/abc099/tasks/abc099_a

fn run(n: usize) -> String {
	if n < 1000 {
		String::from("ABC")
	} else {
		String::from("ABD")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("ABC"), run(999));
		assert_eq!(String::from("ABD"), run(1000));
		assert_eq!(String::from("ABD"), run(1481));
	}
}
