use std::rc::Rc;

use crate::BTreeCache;
use crate::FnCache;

fn square(_cache: &mut BTreeCache<u32,u64>, x: &u32) -> u64 {
	*x as u64 * *x as u64
}

fn fib(cache: &mut BTreeCache<u32,u64>, x: &u32) -> u64 {
	match x {
		0 => 0,
		1 => 1,
		_ => *cache.get(x - 1) + *cache.get(x - 2),
	}
}

#[test]
fn get_fn_ptr() {
	let mut bc = BTreeCache::new(square);

	assert!(!bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);
	assert!(bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);

	assert!(!bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &25);
	assert!(bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &25);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_closure() {
	let mut bc = BTreeCache::<u32,u64>::new(|_cache, &x| x as u64 * x as u64);

	assert!(!bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);
	assert!(bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);

	assert!(!bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &25);
	assert!(bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &25);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_closure_capture() {
	let y = 3;

	let mut bc = BTreeCache::<u32,u64>::new(|_cache, &x| y * x as u64 * x as u64);

	assert!(!bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &3);
	assert!(bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &3);

	assert!(!bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &75);
	assert!(bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &75);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_fn_ptr_recursive() {
	let mut bc = BTreeCache::new(fib);

	assert!(!bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);
	assert!(bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);

	assert!(!bc.cache.contains_key(&2));
	assert!(!bc.cache.contains_key(&3));
	assert!(!bc.cache.contains_key(&4));
	assert!(!bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &5);
	assert!(bc.cache.contains_key(&1));
	assert!(bc.cache.contains_key(&2));
	assert!(bc.cache.contains_key(&3));
	assert!(bc.cache.contains_key(&4));
	assert!(bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &5);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_closure_recursive() {
	let mut bc = BTreeCache::<usize,u64>::new(|cache, x|
		match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1) + *cache.get(x - 2),
		}
	);

	assert!(!bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);
	assert!(bc.cache.contains_key(&1));
	assert_eq!(bc.get(1), &1);

	assert!(!bc.cache.contains_key(&2));
	assert!(!bc.cache.contains_key(&3));
	assert!(!bc.cache.contains_key(&4));
	assert!(!bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &5);
	assert!(bc.cache.contains_key(&1));
	assert!(bc.cache.contains_key(&2));
	assert!(bc.cache.contains_key(&3));
	assert!(bc.cache.contains_key(&4));
	assert!(bc.cache.contains_key(&5));
	assert_eq!(bc.get(5), &5);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn get_alternate_value() {
	let mut bc = BTreeCache::<usize,u64,Rc<u64>>::new(|cache, x|
		match x {
			0 => 0,
			1 => 1,
			_ => *cache.get(x - 1).clone() + *cache.get(x - 2).clone(),
		}
	);

	assert!(!bc.cache.contains_key(&1));
	assert_eq!(*bc.get(1).clone(), 1);
	assert!(bc.cache.contains_key(&1));
	assert_eq!(*bc.get(1).clone(), 1);

	assert!(!bc.cache.contains_key(&2));
	assert!(!bc.cache.contains_key(&3));
	assert!(!bc.cache.contains_key(&4));
	assert!(!bc.cache.contains_key(&5));
	assert_eq!(*bc.get(5).clone(), 5);
	assert!(bc.cache.contains_key(&1));
	assert!(bc.cache.contains_key(&2));
	assert!(bc.cache.contains_key(&3));
	assert!(bc.cache.contains_key(&4));
	assert!(bc.cache.contains_key(&5));
	assert_eq!(*bc.get(5).clone(), 5);

	assert!(bc.cache.contains_key(&1));
}

#[test]
fn clear() {
	let mut bc = BTreeCache::<usize,usize>::new(|_cache, x| *x);

	bc.get(0);
	bc.get(1);
	bc.get(2);

	assert_eq!(bc.len(), 3);

	bc.clear();

	assert_eq!(bc.len(), 0);
}

#[test]
fn len() {
	let mut bc = BTreeCache::<usize,usize>::new(|_cache, x| *x);

	bc.get(0);
	bc.get(1);
	bc.get(2);

	assert_eq!(bc.len(), 3);
}

#[test]
fn remove() {
	let mut bc = BTreeCache::<usize,usize>::new(|_cache, x| *x);

	bc.get(0);
	bc.get(1);
	bc.get(2);

	assert_eq!(bc.len(), 3);
	assert_eq!(bc.remove(&1), Some(1));
	assert_eq!(bc.len(), 2);
	assert_eq!(bc.remove(&1), None);
}
