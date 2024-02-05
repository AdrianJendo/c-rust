pub use sea;

// Entry point for the proof
#[no_mangle]
pub extern "C" fn entrypt() {
    test_test1();
		test_test2();
}

#[no_mangle]
fn test_test1() {
    let mut x: i32 = sea::nd_i32();
    sea::assume(x < 10);
    x += 4;

    sea::sassert!(x < 14);
}

#[no_mangle]
fn test_test2() {
    let x: i32 = sea::nd_i32();
    sea::assume(x < 10);

		let mut m = Mock::new(x);
		m.foo(4);
		m.foo(4);

		assert!(m.times == 2);
		sea::sassert!(m.times == 12);
    sea::sassert!(m.count < 5);
}

trait Foo {
	fn foo(&mut self, count: i32);
}

struct Mock {
	count: i32,
	times: i32,
}

impl Mock {
	fn new(count: i32) -> Mock {
			Mock { count, times: 0 }
	}
}

impl Foo for Mock {
	fn foo(&mut self, x: i32) {
			self.count += x;
			self.times += 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
			let result = 2 + 2;
			assert_eq!(result, 4);
	}

	#[test]
	fn test_mock() {
			let mut x = Mock::new(0);

			assert!(x.times == 0);

			x.foo(2);
			x.foo(2);

			assert!(x.times == 2);
			assert!(x.count == 4)
	}
}