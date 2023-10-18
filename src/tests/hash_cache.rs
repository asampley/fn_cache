use std::fmt::Debug;
use std::hash::{BuildHasher, BuildHasherDefault, Hash};
use std::rc::Rc;

use crate::tests::*;
use crate::HashCache;
use crate::{FnCache, FnCacheMany};

use hashers::fx_hash::FxHasher;

fn test_get<K, T, H, V>(hc: &mut HashCache<K, T, H>, k: K, v: V)
where
	H: BuildHasher,
	K: Hash + Eq + Copy,
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

fn test_get_many<K, T, H, V, const N: usize>(hc: &mut HashCache<K, T, H>, k: [K; N], v: [V; N])
where
	H: BuildHasher,
	K: Hash + Eq + Copy,
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
fn with_hasher() {
	let mut hc = HashCache::with_hasher(BuildHasherDefault::<FxHasher>::default(), square);

	test_get(&mut hc, 1, 1);
	test_get(&mut hc, 5, 25);

	test_get_many(&mut hc, [2, 5, 10], [4, 25, 100]);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_fn_ptr() {
	let mut hc = HashCache::new(square);

	test_get(&mut hc, 1, 1);
	test_get(&mut hc, 5, 25);

	test_get_many(&mut hc, [2, 5, 10], [4, 25, 100]);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_closure() {
	let mut hc = HashCache::new(|&x| x as u64 * x as u64);

	test_get(&mut hc, 1, 1);
	test_get(&mut hc, 5, 25);

	test_get_many(&mut hc, [2, 5, 10], [4, 25, 100]);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_closure_capture() {
	let y = 3;

	let mut hc = HashCache::new(|&x| y * x as u64 * x as u64);

	test_get(&mut hc, 1, 3);
	test_get(&mut hc, 5, 75);

	test_get_many(&mut hc, [2, 5, 10], [12, 75, 300]);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_fn_ptr_recursive() {
	let mut hc = HashCache::recursive(fib);

	test_get(&mut hc, 1, 1);

	assert!(!hc.cache.contains_key(&2));
	assert!(!hc.cache.contains_key(&3));
	assert!(!hc.cache.contains_key(&4));
	assert!(!hc.cache.contains_key(&5));
	test_get(&mut hc, 5, 5);
	assert!(hc.cache.contains_key(&1));
	assert!(hc.cache.contains_key(&2));
	assert!(hc.cache.contains_key(&3));
	assert!(hc.cache.contains_key(&4));
	assert!(hc.cache.contains_key(&5));
	test_get(&mut hc, 5, 5);

	test_get_many(&mut hc, [2, 5, 12], [1, 5, 144]);

	assert!(hc.cache.contains_key(&1));
}

#[test]
fn get_closure_recursive() {
	let mut hc = HashCache::<usize, u64>::recursive(|cache, x| match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	});

	test_get(&mut hc, 1, 1);

	assert!(!hc.cache.contains_key(&2));
	assert!(!hc.cache.contains_key(&3));
	assert!(!hc.cache.contains_key(&4));
	assert!(!hc.cache.contains_key(&5));
	test_get(&mut hc, 5, 5);
	assert!(hc.cache.contains_key(&1));
	assert!(hc.cache.contains_key(&2));
	assert!(hc.cache.contains_key(&3));
	assert!(hc.cache.contains_key(&4));
	assert!(hc.cache.contains_key(&5));
	test_get(&mut hc, 5, 5);

	test_get_many(&mut hc, [2, 5, 12], [1, 5, 144]);

	assert!(hc.cache.contains_key(&1));
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

	test_get(&mut hc, 1, 1);

	assert!(!hc.cache.contains_key(&2));
	assert!(!hc.cache.contains_key(&3));
	assert!(!hc.cache.contains_key(&4));
	assert!(!hc.cache.contains_key(&5));
	test_get(&mut hc, 5, 5);
	assert!(hc.cache.contains_key(&1));
	assert!(hc.cache.contains_key(&2));
	assert!(hc.cache.contains_key(&3));
	assert!(hc.cache.contains_key(&4));
	assert!(hc.cache.contains_key(&5));
	test_get(&mut hc, 5, 5);

	test_get_many(&mut hc, [2, 5, 12], [1, 5, 144]);

	assert!(hc.cache.contains_key(&1));
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
