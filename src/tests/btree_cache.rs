use std::rc::Rc;

use crate::tests::*;
use crate::BTreeCache;
use crate::FnCache;

#[test]
fn get_fn_ptr() {
	let mut bc = BTreeCache::new(square);

	test_square(&mut bc);
}

#[test]
fn get_closure() {
	let mut bc = BTreeCache::new(|&x| x as u64 * x as u64);

	test_square(&mut bc);
}

#[test]
fn get_closure_capture() {
	let y = 3;

	let mut bc = BTreeCache::new(|&x| y * x as u64 * x as u64);

	test_factor_square(&mut bc, 3)
}

// TODO this currently doesn't work
//#[test]
//fn get_fn_ptr_recursive() {
//	let mut bc = BTreeCache::recursive(fib);
//
//	test_fib(&mut bc)
//}

#[test]
fn get_closure_recursive() {
	let mut bc = BTreeCache::<usize, u64>::recursive(|cache, x| match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	});

	test_fib(&mut bc)
}

#[test]
fn get_alternate_value() {
	let mut bc = BTreeCache::<usize, Rc<u64>>::recursive(|cache, x| {
		match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1).clone() + *cache.get(x - 2).clone(),
		}
		.into()
	});

	test_fib(&mut bc)
}

#[test]
fn clear() {
	let mut bc = BTreeCache::new(|x| *x);

	bc.get(0);
	bc.get(1);
	bc.get(2);

	assert_eq!(bc.len(), 3);

	bc.clear();

	assert_eq!(bc.len(), 0);
}

#[test]
fn len() {
	let mut bc = BTreeCache::new(|x| *x);

	bc.get(0);
	bc.get(1);
	bc.get(2);

	assert_eq!(bc.len(), 3);
}

#[test]
fn remove() {
	let mut bc = BTreeCache::new(|x| *x);

	bc.get(0);
	bc.get(1);
	bc.get(2);

	assert_eq!(bc.len(), 3);
	assert_eq!(bc.remove(&1), Some(1));
	assert_eq!(bc.len(), 2);
	assert_eq!(bc.remove(&1), None);
}

#[test]
fn static_context() {
	use once_cell::sync::Lazy;
	use std::sync::Mutex;

	static HC: Lazy<Mutex<BTreeCache<usize, usize>>> =
		Lazy::new(|| Mutex::new(BTreeCache::new(|x| *x)));

	let mut hc = HC.lock().unwrap();

	hc.get(0);
	hc.get(1);
	hc.get(2);
}
