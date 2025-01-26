// https://atcoder.jp/contests/abc076/tasks/abc076_a

fn run(r: isize, g: isize) -> isize {
	g * 2 - r
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(isize, isize, isize);

	#[test]
	fn test() {
		let tests = [
			TestCase(2002, 2017, 2032),
			TestCase(4500, 0, -4500),
		];

		for TestCase(r, g, expected) in tests {
			assert_eq!(run(r, g), expected);
		}
	}
}
