use std::hash::BuildHasherDefault;
use std::rc::Rc;

use crate::tests::*;
use crate::FnCache;
use crate::HashCache;

use hashers::fx_hash::FxHasher;

#[test]
fn with_hasher() {
	let mut hc = HashCache::with_hasher(BuildHasherDefault::<FxHasher>::default(), square);

	test_square(&mut hc);
}

#[test]
fn get_fn_ptr() {
	let mut hc = HashCache::new(square);

	test_square(&mut hc);
}

#[test]
fn get_closure() {
	let mut hc = HashCache::new(|&x| x as u64 * x as u64);

	test_square(&mut hc);
}

#[test]
fn get_closure_capture() {
	let y = 3;

	let mut hc = HashCache::new(|&x| y * x as u64 * x as u64);

	test_factor_square(&mut hc, y);
}

// TODO currently this fails
//#[test]
//fn get_fn_ptr_recursive() {
//
//	let mut hc = HashCache::recursive(fib);
//
//	test_fib(&mut hc);
//}

#[test]
fn get_closure_recursive() {
	let mut hc = HashCache::<usize, u64>::recursive(|cache, x| match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	});

	test_fib(&mut hc)
}

#[test]
fn get_alternate_value() {
	let mut hc = HashCache::<usize, Rc<u64>>::recursive(|cache, x| {
		Rc::new(match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1).clone() + *cache.get(x - 2).clone(),
		})
	});

	test_fib(&mut hc)
}

#[test]
fn clear() {
	let mut hc = HashCache::<usize, usize>::new(|x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);

	hc.clear();

	assert_eq!(hc.len(), 0);
}

#[test]
fn len() {
	let mut hc = HashCache::<usize, usize>::new(|x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);
}

#[test]
fn reserve() {
	let mut hc = HashCache::<usize, usize>::new(|x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	for additional in 20..60 {
		hc.cache.shrink_to_fit();
		hc.reserve(additional);

		assert!(
			hc.len() + additional <= hc.cache.capacity(),
			"len = {}, capacity = {}, additional = {}",
			hc.len(),
			hc.cache.capacity(),
			additional
		);
	}
}

#[test]
fn remove() {
	let mut hc = HashCache::<usize, usize>::new(|x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);
	assert_eq!(hc.remove(&1), Some(1));
	assert_eq!(hc.len(), 2);
	assert_eq!(hc.remove(&1), None);
}

#[test]
fn static_context() {
	use once_cell::sync::Lazy;
	use std::sync::Mutex;

	static HC: Lazy<Mutex<HashCache<usize, usize>>> =
		Lazy::new(|| Mutex::new(HashCache::new(|x| *x)));

	let mut hc = HC.lock().unwrap();

	hc.get(0);
	hc.get(1);
	hc.get(2);
}
