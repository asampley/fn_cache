use std::rc::Rc;

use crate::tests::*;
use crate::FnCache;
use crate::VecCache;

#[test]
fn cache_fn_ptr() {
	let mut vc = VecCache::new(square);

	assert_eq!(vc.cache.len(), 0);
	assert_eq!(vc.get(0), &0);
	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(0), &0);

	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(5), &25);
	assert_eq!(vc.cache.len(), 6);
	assert_eq!(vc.get(5), &25);

	assert_eq!(vc.get(3), &9);
	assert_eq!(vc.cache.len(), 6);
}

#[test]
fn cache_closure() {
	let mut vc = VecCache::<u64>::new(|_cache, x| *x as u64 * *x as u64);

	assert_eq!(vc.cache.len(), 0);
	assert_eq!(vc.get(0), &0);
	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(0), &0);

	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(5), &25);
	assert_eq!(vc.cache.len(), 6);
	assert_eq!(vc.get(5), &25);

	assert_eq!(vc.get(3), &9);
	assert_eq!(vc.cache.len(), 6);
}

#[test]
fn cache_closure_capture() {
	let y = 3;

	let mut vc = VecCache::<u64>::new(|_cache, x| y * *x as u64 * *x as u64);

	assert_eq!(vc.cache.len(), 0);
	assert_eq!(vc.get(0), &0);
	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(0), &0);

	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(5), &75);
	assert_eq!(vc.cache.len(), 6);
	assert_eq!(vc.get(5), &75);

	assert_eq!(vc.get(3), &27);
	assert_eq!(vc.cache.len(), 6);
}

#[test]
fn cache_fn_ptr_recursive() {
	let mut vc = VecCache::new(fib);

	assert_eq!(vc.cache.len(), 0);
	assert_eq!(vc.get(0), &0);
	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(0), &0);

	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(5), &5);
	assert_eq!(vc.cache.len(), 6);
	assert_eq!(vc.get(5), &5);

	assert_eq!(vc.get(3), &2);
	assert_eq!(vc.cache.len(), 6);
}

#[test]
fn cache_closure_recursive() {
	let mut vc = VecCache::<u64>::new(|cache, x| match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	});

	assert_eq!(vc.cache.len(), 0);
	assert_eq!(vc.get(0), &0);
	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(0), &0);

	assert_eq!(vc.cache.len(), 1);
	assert_eq!(vc.get(5), &5);
	assert_eq!(vc.cache.len(), 6);
	assert_eq!(vc.get(5), &5);

	assert_eq!(vc.get(3), &2);
	assert_eq!(vc.cache.len(), 6);
}

#[test]
fn cache_alternate_cache() {
	let mut vc = VecCache::<Rc<u64>>::new(|cache, x|
		Rc::new(match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1).clone() + *cache.get(x - 2).clone(),
		})
	);

	assert_eq!(vc.cache.len(), 0);
	assert_eq!(*vc.get(0).clone(), 0);
	assert_eq!(vc.cache.len(), 1);
	assert_eq!(*vc.get(0).clone(), 0);

	assert_eq!(vc.cache.len(), 1);
	assert_eq!(*vc.get(5).clone(), 5);
	assert_eq!(vc.cache.len(), 6);
	assert_eq!(*vc.get(5).clone(), 5);

	assert_eq!(*vc.get(3).clone(), 2);
	assert_eq!(vc.cache.len(), 6);
}

#[test]
fn clear() {
	let mut vc = VecCache::<usize>::new(|_cache, x| *x);

	vc.get(2);

	assert_eq!(vc.cache.len(), 3);

	vc.clear();

	assert_eq!(vc.cache.len(), 0);
}

#[test]
fn len() {
	let mut vc = VecCache::<usize>::new(|_cache, x| *x);

	vc.get(0);
	vc.get(1);
	vc.get(2);

	assert_eq!(vc.len(), 3);
}

#[test]
fn reserve() {
	let mut vc = VecCache::<usize>::new(|_cache, x| *x);

	vc.get(0);
	vc.get(1);
	vc.get(2);

	for additional in 20..60 {
		vc.cache.shrink_to_fit();
		vc.reserve(additional);

		assert!(
			vc.len() + additional <= vc.cache.capacity(),
			"len = {}, capacity = {}, additional = {}",
			vc.len(),
			vc.cache.capacity(),
			additional
		);
	}
}
