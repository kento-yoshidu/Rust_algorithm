// https://atcoder.jp/contests/abc072/tasks/abc072_a

fn run(x: isize, t: isize) -> isize {
	if x <= t {
		0
	} else {
		x - t
	}
}

fn run2(x: isize, t: isize) -> isize {
	std::cmp::max(0, x - t)
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(isize, isize, isize);

	#[test]
	fn test() {
		let tests = [
			TestCase(100, 17, 83),
			TestCase(48, 58, 0),
			TestCase(1000000000, 1000000000, 0),
		];

		for TestCase(x, t, expected) in tests {
			assert_eq!(run(x, t), expected);
			assert_eq!(run2(x, t), expected);
		}
	}
}
