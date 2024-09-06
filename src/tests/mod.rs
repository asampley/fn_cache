#![cfg(test)]
mod btree_cache;
mod hash_cache;
mod vec_cache;

use std::borrow::Borrow;
use std::fmt::Debug;

use crate::container::{ContainerLen, SparseContainer};
use crate::{FnCache, FnCacheMany, GenericCache};

fn test_get<C, V>(hc: &mut GenericCache<C>, k: C::Input, v: V)
where
	C: SparseContainer + ContainerLen,
	C::Input: Copy,
	V: Debug,
	C::Output: Borrow<V>,
	for<'a> &'a V: PartialEq,
{
	let len = hc.len();
	let minimum_len = len + if !hc.cache().has(&k) { 1 } else { 0 };

	assert_eq!(hc.get(k).borrow(), &v);
	assert!(hc.cache().has(&k));
	assert!(hc.len() >= minimum_len);
	assert_eq!(hc.get(k).borrow(), &v);
	assert!(hc.cache().has(&k));
	assert!(hc.len() >= minimum_len);
}

fn test_get_many<C, V, const N: usize>(hc: &mut GenericCache<C>, k: [C::Input; N], v: [V; N])
where
	C: SparseContainer + ContainerLen,
	C::Input: Copy,
	C::Output: Borrow<V>,
	V: Debug,
	for<'a> &'a V: PartialEq,
{
	let len = hc.len();
	let minimum_len = len + k.iter().filter(|x| !hc.cache().has(x)).count();

	let refs = std::array::from_fn(|i| &v[i]);

	assert_eq!(hc.get_many(k).map(|x| x.borrow()), refs);
	assert!(hc.len() >= minimum_len);
	assert_eq!(hc.get_many(k).map(|x| x.borrow()), refs);
	assert!(hc.len() >= minimum_len);
}

fn test_square<C>(cache: &mut GenericCache<C>)
where
	C: SparseContainer<Input = usize, Output = u64> + ContainerLen,
{
	test_factor_square(cache, 1)
}

fn test_factor_square<C>(cache: &mut GenericCache<C>, factor: u64)
where
	C: SparseContainer<Input = usize, Output = u64> + ContainerLen,
{
	test_get(cache, 1, factor * 1);
	test_get(cache, 5, factor * 25);

	test_get_many(cache, [2, 5, 10], [4, 25, 100].map(|x| factor * x));

	assert!(cache.cache().has(&1));
}

fn test_fib<C>(cache: &mut GenericCache<C>)
where
	C: SparseContainer<Input = usize> + ContainerLen,
	C::Output: Borrow<u64>,
{
	test_get(cache, 1, 1);

	assert!(!cache.cache().has(&2));
	assert!(!cache.cache().has(&3));
	assert!(!cache.cache().has(&4));
	assert!(!cache.cache().has(&5));
	test_get(cache, 5, 5);
	assert!(cache.cache().has(&1));
	assert!(cache.cache().has(&2));
	assert!(cache.cache().has(&3));
	assert!(cache.cache().has(&4));
	assert!(cache.cache().has(&5));
	test_get(cache, 5, 5);

	test_get_many(cache, [2, 5, 12], [1, 5, 144]);

	assert!(cache.cache().has(&1));
}

fn square(x: &usize) -> u64 {
	*x as u64 * *x as u64
}

fn fib(cache: &mut impl FnCache<usize, u64>, x: &usize) -> u64 {
	match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	}
}
