// https://atcoder.jp/contests/abc078/tasks/abc078_a

fn run(x: char, y: char) -> char {
	let a = x as usize;
	let b = y as usize;

	if a < b {
		'<'
	} else if a == b {
		'='
	} else {
		'>'
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!('<', run('A', 'B'));
		assert_eq!('>', run('E', 'C'));
		assert_eq!('=', run('F', 'F'));
	}
}
