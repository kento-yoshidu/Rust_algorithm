// https://atcoder.jp/contests/abc098/tasks/abc098_a

fn run(a: isize, b: isize) -> isize {
	*[a + b, a - b, a * b].iter().max().unwrap()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(4, run(3, 1));
		assert_eq!(6, run(4, -2));
		assert_eq!(0, run(0, 0));
	}
}
