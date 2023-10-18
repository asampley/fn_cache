use std::fmt::Debug;
use std::rc::Rc;

use crate::tests::*;
use crate::BTreeCache;
use crate::{FnCache, FnCacheMany};

fn test_get<K, T, V>(hc: &mut BTreeCache<K, T>, k: K, v: V)
where
	K: Ord + Eq + Copy,
	V: Debug,
	T: std::borrow::Borrow<V>,
	for<'a> &'a V: PartialEq,
{
	let len = hc.cache.len();
	let minimum_len = len + if !hc.cache.contains_key(&k) { 1 } else { 0 };

	assert_eq!(hc.get(k).borrow(), &v);
	assert!(hc.cache.contains_key(&k));
	assert!(hc.cache.len() >= minimum_len);
	assert_eq!(hc.get(k).borrow(), &v);
	assert!(hc.cache.contains_key(&k));
	assert!(hc.cache.len() >= minimum_len);
}

fn test_get_many<K, T, V, const N: usize>(hc: &mut BTreeCache<K, T>, k: [K; N], v: [V; N])
where
	K: Ord + Eq + Copy,
	V: Debug,
	T: std::borrow::Borrow<V>,
	V: Clone + Debug + PartialEq,
{
	let len = hc.cache.len();
	let minimum_len = len + k.iter().filter(|x| !hc.cache.contains_key(x)).count();

	let refs = std::array::from_fn(|i| &v[i]);

	assert_eq!(hc.get_many(k).map(|x| x.borrow()), refs);
	assert!(hc.cache.len() >= minimum_len);
	assert_eq!(hc.get_many(k).map(|x| x.borrow()), refs);
	assert!(hc.cache.len() >= minimum_len);
}

#[test]
fn get_fn_ptr() {
	let mut bc = BTreeCache::new(square);

	test_get(&mut bc, 1, 1);
	test_get(&mut bc, 5, 25);

	test_get_many(&mut bc, [2, 5, 10], [4, 25, 100]);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_closure() {
	let mut bc = BTreeCache::new(|&x| x as u64 * x as u64);

	test_get(&mut bc, 1, 1);
	test_get(&mut bc, 5, 25);

	test_get_many(&mut bc, [2, 5, 10], [4, 25, 100]);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_closure_capture() {
	let y = 3;

	let mut bc = BTreeCache::new(|&x| y * x as u64 * x as u64);

	test_get(&mut bc, 1, 3);
	test_get(&mut bc, 5, 75);

	test_get_many(&mut bc, [2, 5, 10], [12, 75, 300]);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_fn_ptr_recursive() {
	let mut bc = BTreeCache::recursive(fib);

	test_get(&mut bc, 1, 1);

	assert!(!bc.cache.contains_key(&2));
	assert!(!bc.cache.contains_key(&3));
	assert!(!bc.cache.contains_key(&4));
	assert!(!bc.cache.contains_key(&5));
	test_get(&mut bc, 5, 5);
	assert!(bc.cache.contains_key(&1));
	assert!(bc.cache.contains_key(&2));
	assert!(bc.cache.contains_key(&3));
	assert!(bc.cache.contains_key(&4));
	assert!(bc.cache.contains_key(&5));
	test_get(&mut bc, 5, 5);

	test_get_many(&mut bc, [2, 5, 12], [1, 5, 144]);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_closure_recursive() {
	let mut bc = BTreeCache::<usize, u64>::recursive(|cache, x| match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	});

	test_get(&mut bc, 1, 1);

	assert!(!bc.cache.contains_key(&2));
	assert!(!bc.cache.contains_key(&3));
	assert!(!bc.cache.contains_key(&4));
	assert!(!bc.cache.contains_key(&5));
	test_get(&mut bc, 5, 5);
	assert!(bc.cache.contains_key(&1));
	assert!(bc.cache.contains_key(&2));
	assert!(bc.cache.contains_key(&3));
	assert!(bc.cache.contains_key(&4));
	assert!(bc.cache.contains_key(&5));
	test_get(&mut bc, 5, 5);

	test_get_many(&mut bc, [2, 5, 12], [1, 5, 144]);

	assert!(bc.cache.contains_key(&1));
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

	test_get(&mut bc, 1, 1);

	assert!(!bc.cache.contains_key(&2));
	assert!(!bc.cache.contains_key(&3));
	assert!(!bc.cache.contains_key(&4));
	assert!(!bc.cache.contains_key(&5));
	test_get(&mut bc, 5, 5);
	assert!(bc.cache.contains_key(&1));
	assert!(bc.cache.contains_key(&2));
	assert!(bc.cache.contains_key(&3));
	assert!(bc.cache.contains_key(&4));
	assert!(bc.cache.contains_key(&5));
	test_get(&mut bc, 5, 5);

	test_get_many(&mut bc, [2, 5, 12], [1, 5, 144]);

	assert!(bc.cache.contains_key(&1));
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
