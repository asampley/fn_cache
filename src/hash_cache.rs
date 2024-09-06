use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use core::cmp::Eq;
use core::hash::BuildHasher;
use core::hash::Hash;

use derive_more::derive::{Deref, DerefMut, From};

use crate::container::{
	ContainerClear, ContainerLen, ContainerRemove, ContainerReserve, SparseContainer,
};
use crate::generic_cache::{GenericCache, RefCache};

/// A cache for a function which uses a [`HashMap`].
///
/// The cache takes ownership of all inputs, but
/// only passes a reference to the function,
/// allowing it to store the input in the cache
/// without any copies or clones.
///
/// The requirements for a `HashMap` must be met,
/// specifically the keys must implement `Eq` and
/// `Hash`, and the following propery must hold:
///
/// ```k1 == k2 -> hash(k1) == hash(k2)```
#[derive(Deref, DerefMut, From)]
pub struct HashCache<'f, I, O, S = RandomState>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	raw: GenericCache<'f, HashMap<I, O, S>>,
}

impl<'f, I, O> HashCache<'f, I, O, RandomState>
where
	I: Eq + Hash,
{
	pub fn new(f: impl Fn(&I) -> O + Send + 'f) -> Self {
		Self {
			raw: GenericCache::new(f),
		}
	}

	pub fn recursive(f: impl Fn(&mut RefCache<HashMap<I, O>>, &I) -> O + Send + 'f) -> Self {
		Self {
			raw: GenericCache::recursive(f),
		}
	}
}

impl<'f, I, O, S> HashCache<'f, I, O, S>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	pub fn with_hasher(hash_builder: S, f: impl Fn(&I) -> O + Send + 'f) -> Self {
		Self {
			raw: GenericCache::with_cache(HashMap::with_hasher(hash_builder), f),
		}
	}

	pub fn recursive_with_hasher(
		hash_builder: S,
		f: impl Fn(&mut RefCache<HashMap<I, O, S>>, &I) -> O + Send + 'f,
	) -> Self {
		Self {
			raw: GenericCache::recursive_with_cache(HashMap::with_hasher(hash_builder), f),
		}
	}
}

impl<I, O, S> SparseContainer for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	type Input = I;
	type Output = O;

	fn has(&self, input: &I) -> bool {
		self.contains_key(input)
	}
	fn get(&self, input: &I) -> Option<&O> {
		self.get(input)
	}

	fn put(&mut self, input: I, output: O) -> &O {
		self.entry(input).or_insert(output)
	}
}

impl<I, O, S> ContainerLen for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	fn len(&self) -> usize {
		self.len()
	}
}

impl<I, O, S> ContainerClear for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	fn clear(&mut self) {
		self.clear()
	}
}

impl<I, O, S> ContainerReserve for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl<I, O, S> ContainerRemove for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	fn remove(&mut self, input: &I) -> Option<O> {
		self.remove(input)
	}
}
