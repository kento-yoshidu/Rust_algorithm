// https://atcoder.jp/contests/abc076/tasks/abc076_a

fn run(r: isize, g: isize) -> isize {
	g * 2 - r
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(2032, run(2002, 2017));
		assert_eq!(-4500, run(4500, 0));
	}
}
