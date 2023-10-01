// https://atcoder.jp/contests/abc091/tasks/abc091_a

fn run(a: usize, b: usize, c: usize) -> String {
	if a + b >= c {
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
		assert_eq!(String::from("Yes"), run(50, 100, 120));
		assert_eq!(String::from("No"), run(500, 100, 1000));
		assert_eq!(String::from("No"), run(19, 123, 143));
		assert_eq!(String::from("Yes"), run(19, 123, 142));
	}
}
