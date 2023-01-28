use std::collections::BTreeMap;

use core::cmp::Ord;
use core::ops::Index;
use std::sync::Arc;

use crate::FnCache;

/// A cache for a function which uses a [`BTreeMap`].
///
/// The cache takes ownership of all inputs, but
/// only passes a reference to the function,
/// allowing it to store the input in the cache
/// without any copies or clones.
///
/// The requirements for a [`BTreeMap`] must be met,
/// specifically the keys must implement [`Ord`]
pub struct BTreeCache<'f, I, O>
where
	I: Ord,
{
	pub(crate) cache: BTreeMap<I, O>,
	f: Arc<dyn Fn(&mut Self, &I) -> O + 'f + Send + Sync>,
}

impl<'f, I, O> FnCache<I, O> for BTreeCache<'f, I, O>
where
	I: Ord,
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

impl<'f, I, O> BTreeCache<'f, I, O>
where
	I: Ord,
{
	/// Create a cache for the provided function. If the
	/// function stores references, the cache can only
	/// live as long as those references.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&I) -> O + 'f + Send + Sync,
	{
		Self::recursive(move |_, x| f(x))
	}

	/// Create a cache for the provided recursive function.
	/// If the function stores references, the cache can
	/// only live as long as those references.
	pub fn recursive<F>(f: F) -> Self
	where
		F: Fn(&mut Self, &I) -> O + 'f + Send + Sync,
	{
		Self {
			cache: BTreeMap::default(),
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

	/// Removes the input from the cache, returning any value
	/// if the input was previously in the cache.
	pub fn remove(&mut self, input: &I) -> Option<O> {
		self.cache.remove(input)
	}
}
