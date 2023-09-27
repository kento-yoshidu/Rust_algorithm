// https://atcoder.jp/contests/abc070/tasks/abc070_a

fn run(s: String) -> String {
	if s.chars().eq(s.chars().rev()) {
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
		assert_eq!(String::from("Yes"), run(String::from("575")));
		assert_eq!(String::from("No"), run(String::from("123")));
		assert_eq!(String::from("No"), run(String::from("812")));
	}
}
