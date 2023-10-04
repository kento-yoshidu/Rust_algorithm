// https://atcoder.jp/contests/abc072/tasks/abc072_a

pub fn run(x: isize, t: isize) -> isize {
	if x <= t {
		0
	} else {
		x - t
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(83, run(100, 17));
		assert_eq!(0, run(48, 58));
		assert_eq!(0, run(1000000000, 1000000000));
	}
}
