use std::rc::Rc;

use crate::HashCache;
use crate::FnCache;
use crate::tests::*;

#[test]
fn get_fn_ptr() {
	let mut hc = HashCache::new(square);

	assert!(!hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);
	assert!(hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);

	assert!(!hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &25);
	assert!(hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &25);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_closure() {
	let mut hc = HashCache::<usize,u64>::new(|_cache, &x| x as u64 * x as u64);

	assert!(!hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);
	assert!(hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);

	assert!(!hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &25);
	assert!(hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &25);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_closure_capture() {
	let y = 3;

	let mut hc = HashCache::<usize,u64>::new(|_cache, &x| y * x as u64 * x as u64);

	assert!(!hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &3);
	assert!(hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &3);

	assert!(!hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &75);
	assert!(hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &75);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_fn_ptr_recursive() {
	let mut hc = HashCache::new(fib);

	assert!(!hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);
	assert!(hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);

	assert!(!hc.cache.contains_key(&2));
	assert!(!hc.cache.contains_key(&3));
	assert!(!hc.cache.contains_key(&4));
	assert!(!hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &5);
	assert!(hc.cache.contains_key(&1));
	assert!(hc.cache.contains_key(&2));
	assert!(hc.cache.contains_key(&3));
	assert!(hc.cache.contains_key(&4));
	assert!(hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &5);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_closure_recursive() {
	let mut hc = HashCache::<usize,u64>::new(|cache, x|
		match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1) + *cache.get(x - 2),
		}
	);

	assert!(!hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);
	assert!(hc.cache.contains_key(&1));
	assert_eq!(hc.get(1), &1);

	assert!(!hc.cache.contains_key(&2));
	assert!(!hc.cache.contains_key(&3));
	assert!(!hc.cache.contains_key(&4));
	assert!(!hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &5);
	assert!(hc.cache.contains_key(&1));
	assert!(hc.cache.contains_key(&2));
	assert!(hc.cache.contains_key(&3));
	assert!(hc.cache.contains_key(&4));
	assert!(hc.cache.contains_key(&5));
	assert_eq!(hc.get(5), &5);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_alternate_value() {
	let mut hc = HashCache::<usize,u64,Rc<u64>>::new(|cache, x|
		match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1).clone() + *cache.get(x - 2).clone(),
		}
	);

	assert!(!hc.cache.contains_key(&1));
	assert_eq!(*hc.get(1).clone(), 1);
	assert!(hc.cache.contains_key(&1));
	assert_eq!(*hc.get(1).clone(), 1);

	assert!(!hc.cache.contains_key(&2));
	assert!(!hc.cache.contains_key(&3));
	assert!(!hc.cache.contains_key(&4));
	assert!(!hc.cache.contains_key(&5));
	assert_eq!(*hc.get(5).clone(), 5);
	assert!(hc.cache.contains_key(&1));
	assert!(hc.cache.contains_key(&2));
	assert!(hc.cache.contains_key(&3));
	assert!(hc.cache.contains_key(&4));
	assert!(hc.cache.contains_key(&5));
	assert_eq!(*hc.get(5).clone(), 5);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn clear() {
	let mut hc = HashCache::<usize,usize>::new(|_cache, x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);

	hc.clear();

	assert_eq!(hc.len(), 0);
}

#[test]
fn len() {
	let mut hc = HashCache::<usize,usize>::new(|_cache, x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);
}

#[test]
fn reserve() {
	let mut hc = HashCache::<usize,usize>::new(|_cache, x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	for additional in 20..60 {
		hc.cache.shrink_to_fit();
		hc.reserve(additional);

		assert!(
			hc.len() + additional <= hc.cache.capacity(),
			"len = {}, capacity = {}, additional = {}",
			hc.len(), hc.cache.capacity(), additional
		);
	}
}

#[test]
fn remove() {
	let mut hc = HashCache::<usize,usize>::new(|_cache, x| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);
	assert_eq!(hc.remove(&1), Some(1));
	assert_eq!(hc.len(), 2);
	assert_eq!(hc.remove(&1), None);
}
