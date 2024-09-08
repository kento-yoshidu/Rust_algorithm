// https://atcoder.jp/contests/abc084/tasks/abc084_a

fn run(m: usize) -> usize {
	(24 - m) + 24
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(27, run(21));
		assert_eq!(36, run(12));
	}
}
