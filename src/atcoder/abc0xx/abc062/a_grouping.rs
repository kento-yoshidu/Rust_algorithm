// https://atcoder.jp/contests/abc062/tasks/abc062_a

fn run(x: usize, y: usize) -> &'static str {
	let g = [0, 2, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0];

	if g[x-1] == g[y-1] {
		"Yes"
	} else {
		"No"
	}

}

fn check(num: usize) -> usize {
	match num {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 0,
		4 | 6 | 9 | 11 => 1,
		2 => 2,
		_ => unreachable!()
	}
}

pub fn run2(x: usize, y: usize) -> &'static str {
	if check(x) == check(y) {
		"Yes"
	} else {
		"No"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase(1, 3, "Yes"),
			TestCase(2, 4, "No"),
		];

		for TestCase(x, y, expected) in tests {
			assert_eq!(run(x, y), expected);
			assert_eq!(run2(x, y), expected);
		}
	}
}
