use std::cmp::max;
use std::fmt::Debug;
use std::rc::Rc;

use crate::tests::*;
use crate::VecCache;
use crate::{FnCache, FnCacheMany};

fn test_get<T, V>(vc: &mut VecCache<T>, n: usize, v: V)
where
	V: Debug,
	T: std::borrow::Borrow<V>,
	for<'a> &'a V: PartialEq,
{
	let len = vc.cache.len();
	let expected_len = max(len, n + 1);

	assert_eq!(vc.get(n).borrow(), &v);
	assert_eq!(vc.cache.len(), expected_len);
	assert_eq!(vc.get(n).borrow(), &v);
	assert_eq!(vc.cache.len(), expected_len);
}

fn test_get_many<T, V, const N: usize>(vc: &mut VecCache<T>, n: [usize; N], v: [V; N])
where
	V: Debug,
	T: std::borrow::Borrow<V>,
	V: Clone + Debug + PartialEq,
{
	let len = vc.cache.len();
	let expected_len = max(len, n.iter().copied().max().unwrap_or(0) + 1);

	let refs = std::array::from_fn(|i| &v[i]);

	assert_eq!(vc.get_many(n).map(|x| x.borrow()), refs);
	assert_eq!(vc.cache.len(), expected_len);
	assert_eq!(vc.get_many(n).map(|x| x.borrow()), refs);
	assert_eq!(vc.cache.len(), expected_len);
}

#[test]
fn cache_fn_ptr() {
	let mut vc = VecCache::new(square);

	test_get(&mut vc, 0, 0);
	test_get(&mut vc, 5, 25);
	test_get(&mut vc, 3, 9);

	test_get_many(&mut vc, [0, 5, 3], [0, 25, 9]);
	test_get_many(&mut vc, [0, 7, 5, 3], [0, 49, 25, 9]);
	test_get_many(&mut vc, [8, 0, 5, 3], [64, 0, 25, 9]);
	test_get_many(&mut vc, [0, 5, 3, 12], [0, 25, 9, 144]);
}

#[test]
fn cache_closure() {
	let mut vc = VecCache::<u64>::new(|x| *x as u64 * *x as u64);

	test_get(&mut vc, 0, 0);
	test_get(&mut vc, 5, 25);
	test_get(&mut vc, 3, 9);

	test_get_many(&mut vc, [0, 5, 3], [0, 25, 9]);
	test_get_many(&mut vc, [0, 7, 5, 3], [0, 49, 25, 9]);
	test_get_many(&mut vc, [8, 0, 5, 3], [64, 0, 25, 9]);
	test_get_many(&mut vc, [0, 5, 3, 12], [0, 25, 9, 144]);
}

#[test]
fn cache_closure_capture() {
	let y = 3;

	let mut vc = VecCache::<u64>::new(|x| y * *x as u64 * *x as u64);

	test_get(&mut vc, 0, 0);
	test_get(&mut vc, 5, 75);
	test_get(&mut vc, 3, 27);

	test_get_many(&mut vc, [0, 5, 3], [0, 75, 27]);
	test_get_many(&mut vc, [0, 7, 5, 3], [0, 147, 75, 27]);
	test_get_many(&mut vc, [8, 0, 5, 3], [192, 0, 75, 27]);
	test_get_many(&mut vc, [0, 5, 3, 12], [0, 75, 27, 432]);
}

#[test]
fn cache_fn_ptr_recursive() {
	let mut vc = VecCache::recursive(fib);

	test_get(&mut vc, 0, 0);
	test_get(&mut vc, 5, 5);
	test_get(&mut vc, 3, 2);

	test_get_many(&mut vc, [0, 5, 3], [0, 5, 2]);
	test_get_many(&mut vc, [0, 7, 5, 3], [0, 13, 5, 2]);
	test_get_many(&mut vc, [8, 0, 5, 3], [21, 0, 5, 2]);
	test_get_many(&mut vc, [0, 5, 3, 12], [0, 5, 2, 144]);
}

#[test]
fn cache_closure_recursive() {
	let mut vc = VecCache::<u64>::recursive(|cache, x| match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	});

	test_get(&mut vc, 0, 0);
	test_get(&mut vc, 5, 5);
	test_get(&mut vc, 3, 2);

	test_get_many(&mut vc, [0, 5, 3], [0, 5, 2]);
	test_get_many(&mut vc, [0, 7, 5, 3], [0, 13, 5, 2]);
	test_get_many(&mut vc, [8, 0, 5, 3], [21, 0, 5, 2]);
	test_get_many(&mut vc, [0, 5, 3, 12], [0, 5, 2, 144]);
}

#[test]
fn cache_alternate_cache() {
	let mut vc = VecCache::<Rc<u64>>::recursive(|cache, x| {
		Rc::new(match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1).clone() + *cache.get(x - 2).clone(),
		})
	});

	test_get(&mut vc, 0, 0);
	test_get(&mut vc, 5, 5);
	test_get(&mut vc, 3, 2);

	test_get_many(&mut vc, [0, 5, 3], [0, 5, 2]);
	test_get_many(&mut vc, [0, 7, 5, 3], [0, 13, 5, 2]);
	test_get_many(&mut vc, [8, 0, 5, 3], [21, 0, 5, 2]);
	test_get_many(&mut vc, [0, 5, 3, 12], [0, 5, 2, 144]);
}

#[test]
fn clear() {
	let mut vc = VecCache::<usize>::new(|x| *x);

	vc.get(2);

	assert_eq!(vc.cache.len(), 3);

	vc.clear();

	assert_eq!(vc.cache.len(), 0);
}

#[test]
fn len() {
	let mut vc = VecCache::<usize>::new(|x| *x);

	vc.get(0);
	vc.get(1);
	vc.get(2);

	assert_eq!(vc.len(), 3);
}

#[test]
fn reserve() {
	let mut vc = VecCache::<usize>::new(|x| *x);

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

#[test]
fn static_context() {
	use once_cell::sync::Lazy;
	use std::sync::Mutex;

	static VC: Lazy<Mutex<VecCache<usize>>> = Lazy::new(|| Mutex::new(VecCache::new(|x| *x)));

	let mut vc = VC.lock().unwrap();

	test_get(&mut *vc, 0, 0);
	test_get(&mut *vc, 5, 5);
	test_get(&mut *vc, 3, 3);

	test_get_many(&mut *vc, [0, 5, 3], [0, 5, 3]);
	test_get_many(&mut *vc, [0, 7, 5, 3], [0, 7, 5, 3]);
	test_get_many(&mut *vc, [8, 0, 5, 3], [8, 0, 5, 3]);
	test_get_many(&mut *vc, [0, 5, 3, 12], [0, 5, 3, 12]);
}
