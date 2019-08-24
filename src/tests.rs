#![cfg(test)]
use super::*;
use std::rc::Rc;

mod hash_cache {
	use super::*;

	fn square(_cache: &mut HashCache<u32,u64>, x: &u32) -> u64 {
		*x as u64 * *x as u64
	}

	fn fib(cache: &mut HashCache<u32,u64>, x: &u32) -> u64 {
		match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1) + *cache.get(x - 2),
		}
	}

	#[test]
	fn cache_fn_ptr() {
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
	fn cache_closure() {
		let mut hc = HashCache::<u32,u64>::new(|_cache, &x| x as u64 * x as u64);

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
	fn cache_closure_capture() {
		let y = 3;

		let mut hc = HashCache::<u32,u64>::new(|_cache, &x| y * x as u64 * x as u64);

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
	fn cache_fn_ptr_recursive() {
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
	fn cache_closure_recursive() {
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
	fn cache_alternate_cache() {
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
}

mod vec_cache {
	use super::*;

	fn square(_cache: &mut VecCache<u64>, x: usize) -> u64 {
		x as u64 * x as u64
	}

	fn fib(cache: &mut VecCache<u64>, x: usize) -> u64 {
		match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1) + *cache.get(x - 2),
		}
	}

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
		let mut vc = VecCache::<u64>::new(|_cache, x| x as u64 * x as u64);

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

		let mut vc = VecCache::<u64>::new(|_cache, x| y * x as u64 * x as u64);

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
		let mut vc = VecCache::<u64>::new(|cache, x|
			match x {
				0 => 0,
				1 => 1,
				_ => *cache.get(x - 1) + *cache.get(x - 2),
			}
		);

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
		let mut vc = VecCache::<u64,Rc<u64>>::new(|cache, x|
			match x {
				0 => 0,
				1 => 1,
				_ => *cache.get(x - 1).clone() + *cache.get(x - 2).clone(),
			}
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
}
