// https://atcoder.jp/contests/abc094/tasks/abc094_a

fn run(a: usize, b: usize, x: usize) -> String {
	if a > x || a + b < x {
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
		assert_eq!(String::from("Yes"), run(3, 5, 4));
		assert_eq!(String::from("No"), run(2, 2, 6));
		assert_eq!(String::from("No"), run(5, 3, 2));
	}
}
