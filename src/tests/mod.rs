#![cfg(test)]
mod btree_cache;
mod hash_cache;
mod vec_cache;

use crate::FnCache;

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
