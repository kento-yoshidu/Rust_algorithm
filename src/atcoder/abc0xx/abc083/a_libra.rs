// https://atcoder.jp/contests/abc083/tasks/abc083_a

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
	if a + b > c + d {
		"Left"
	} else if a + b == c + d {
		"Balanced"
	} else {
		"Right"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize, usize, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase(3, 8, 7, 1, "Left"),
			TestCase(3, 4, 5, 2, "Balanced"),
			TestCase(1, 7, 6, 4, "Right"),
		];

		for TestCase(a, b, c, d, expected) in tests {
			assert_eq!(run(a, b, c, d), expected);
		}
	}
}
