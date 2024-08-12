use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, RandomState};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::raw_cache::{FnRecursive, FnSimple};
use crate::tests::*;
use crate::RawCache;
use crate::{FnCache, FnCacheMany};

use hashers::fx_hash::FxHasher;

//type HashCache<K, V, H = RandomState> = RawCache<HashMap<K, V, H>, K, V>;

struct HashCache<'f, I, O, S = RandomState>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	raw: RawCache<'f, HashMap<I, O, S>>,
}

impl<'f, I, O> HashCache<'f, I, O, RandomState>
where
	I: Eq + Hash,
{
	fn new(f: impl FnSimple<HashMap<I, O>> + 'f) -> Self {
		Self { raw: RawCache::new(f) }
	}

	fn recursive(f: impl FnRecursive<HashMap<I, O>> + 'f) -> Self {
		Self { raw: RawCache::recursive(f) }
	}
}

impl<'f, I, O, S> HashCache<'f, I, O, S>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	fn with_hasher(hash_builder: S, f: impl FnSimple<HashMap<I, O, S>> + 'f) -> Self {
		Self { raw: RawCache::with_cache(HashMap::with_hasher(hash_builder), f) }
	}

	fn recursive_with_hasher(hash_builder: S, f: impl FnRecursive<HashMap<I, O, S>> + 'f) -> Self {
		Self { raw: RawCache::recursive_with_cache(HashMap::with_hasher(hash_builder), f) }
	}
}

impl<'f, I, O, S> Deref for HashCache<'f, I, O, S>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	type Target = RawCache<'f, HashMap<I, O, S>>;

	fn deref(&self) -> &Self::Target {
		&self.raw
	}
}

impl<'f, I, O, S> DerefMut for HashCache<'f, I, O, S>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.raw
	}
}

fn test_get<'c, K, V, H, T>(hc: &mut HashCache<K, V, H>, k: K, v: T)
where
	H: BuildHasher,
	K: Hash + Eq + Copy,
	T: Debug,
	V: std::borrow::Borrow<T>,
	for<'a> &'a T: PartialEq,
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
	let mut hc = HashCache::recursive(|cache, x| match x {
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
	let mut hc = HashCache::new(|x: &usize| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);

	hc.clear();

	assert_eq!(hc.len(), 0);
}

#[test]
fn len() {
	let mut hc = HashCache::new(|x: &usize| *x);

	hc.get(0);
	hc.get(1);
	hc.get(2);

	assert_eq!(hc.len(), 3);
}

#[test]
fn reserve() {
	let mut hc = HashCache::new(|x: &usize| *x);

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
	let mut hc = HashCache::new(|x: &usize| *x);

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
