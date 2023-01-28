use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use core::cmp::Eq;
use core::hash::BuildHasher;
use core::hash::Hash;
use core::ops::Index;
use std::sync::Arc;

use crate::FnCache;

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
pub struct HashCache<'f, I, O, S = RandomState>
where
	I: Eq + Hash,
{
	pub(crate) cache: HashMap<I, O, S>,
	f: Arc<dyn Fn(&mut Self, &I) -> O + 'f + Send + Sync>,
}

impl<'f, I, O, S> FnCache<I, O> for HashCache<'f, I, O, S>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	fn get(&mut self, input: I) -> &O {
		if self.cache.contains_key(&input) {
			self.cache.index(&input)
		} else {
			let output = self.compute(&input);
			self.cache.entry(input).or_insert(output.into())
		}
	}
}

impl<'f, I, O> HashCache<'f, I, O, RandomState>
where
	I: Eq + Hash,
{
	/// Create a cache for the provided function. If the
	/// function stores references, the cache can only
	/// live as long as those references.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&I) -> O + 'f + Send + Sync,
	{
		Self::recursive(move |_, i| f(i))
	}

	/// Create a cache for the provided recursive function.
	/// If the function stores references, the cache can
	/// only live as long as those references.
	pub fn recursive<F>(f: F) -> Self
	where
		F: Fn(&mut Self, &I) -> O + 'f + Send + Sync,
	{
		HashCache {
			cache: HashMap::default(),
			f: Arc::new(f),
		}
	}
}

impl<'f, I, O, S> HashCache<'f, I, O, S>
where
	I: Eq + Hash,
	S: BuildHasher,
{
	/// Create a HashCache which will use the given hash
	/// builder to hash keys.
	///
	/// See the documentation on [`HashMap::with_hasher`] for more details.
	pub fn with_hasher<F>(hash_builder: S, f: F) -> Self
	where
		F: Fn(&I) -> O + 'f + Send + Sync,
	{
		Self::recursive_with_hasher(hash_builder, move |_, i| f(i))
	}

	/// Create a recursive HashCache which will use the given hash
	/// builder to hash keys.
	///
	/// See the documentation on [`HashMap::with_hasher`] for more details.
	pub fn recursive_with_hasher<F>(hash_builder: S, f: F) -> Self
	where
		F: Fn(&mut Self, &I) -> O + 'f + Send + Sync,
	{
		HashCache {
			cache: HashMap::with_hasher(hash_builder),
			f: Arc::new(f),
		}
	}

	fn compute(&mut self, input: &I) -> O {
		(self.f.clone())(self, input)
	}

	/// Clears the cache, removing all key-value pairs.
	/// Keeps the allocated memory for reuse.
	pub fn clear(&mut self) {
		self.cache.clear();
	}

	/// Returns the number of elements in the cache.
	pub fn len(&self) -> usize {
		self.cache.len()
	}

	/// Reserves capacity for at least `additional` more elements
	/// to be inserted in the cache. The collection may
	/// reserve more space to avoid frequent reallocations.
	pub fn reserve(&mut self, additional: usize) {
		self.cache.reserve(additional)
	}

	/// Removes the input from the cache, returning any value
	/// if the input was previously in the cache.
	pub fn remove(&mut self, input: &I) -> Option<O> {
		self.cache.remove(input)
	}
}
