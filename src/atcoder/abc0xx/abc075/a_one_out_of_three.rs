// https://atcoder.jp/contests/abc075/tasks/abc075_a

fn run(a: isize, b: isize, c: isize) -> isize {
	a ^ b ^ c
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(isize, isize, isize, isize);

	#[test]
	fn test() {
		let tests = [
			TestCase(5, 7, 5, 7),
			TestCase(1, 1, 7, 7),
			TestCase(-100, 100, 100, -100),
		];

		for TestCase(a, b, c, expected) in tests {
			assert_eq!(run(a, b, c), expected);
		}
	}
}
