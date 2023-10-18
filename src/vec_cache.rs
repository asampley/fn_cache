use crate::{FnCache, FnCacheMany};

use std::sync::Arc;

/// A cache for a function which uses a [`Vec`].
///
/// This cache is optimized for functions which must
/// be calculated in order, so that there can be no
/// gaps in the cache, and use `usize` as an argument.
///
/// If the function does not start at zero, or require
/// every previous value to be calculated for the next
/// one, consider using a [`HashCache`](crate::HashCache)
/// instead.
pub struct VecCache<'f, O> {
	pub(crate) cache: Vec<O>,
	f: Arc<dyn Fn(&mut Self, &usize) -> O + 'f + Send + Sync>,
}

impl<'f, O> FnCache<usize, O> for VecCache<'f, O> {
	fn get(&mut self, input: usize) -> &O {
		let len = self.cache.len();

		if len <= input {
			self.cache.reserve(input - len + 1);
		}

		while self.cache.len() <= input {
			let next = self.cache.len();
			let next_val = self.compute(next);
			self.cache.push(next_val);
		}

		self.cache.get(input).unwrap()
	}
}

impl<'f, O> FnCacheMany<usize, O> for VecCache<'f, O> {
	fn get_many<const N: usize>(&mut self, inputs: [usize; N]) -> [&O; N] {
		let len = self.cache.len();
		let max = inputs.iter().max().copied().unwrap_or(0);

		if len <= max {
			self.cache.reserve(max - len + 1);
		}

		for i in inputs {
			self.get(i);
		}

		inputs.map(|i| self.cache.get(i).unwrap())
	}
}

impl<'f, O> VecCache<'f, O> {
	/// Create a cache for the provided function. If the
	/// function stores references, the cache can only
	/// live as long as those references.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&usize) -> O + 'f + Send + Sync,
	{
		Self::recursive(move |_, x| f(x))
	}

	/// Create a cache for the provided recursive function.
	/// If the function stores references, the cache can
	/// only live as long as those references.
	pub fn recursive<F>(f: F) -> Self
	where
		F: Fn(&mut Self, &usize) -> O + 'f + Send + Sync,
	{
		VecCache {
			cache: Vec::default(),
			f: Arc::new(f),
		}
	}

	fn compute(&mut self, input: usize) -> O {
		(self.f.clone())(self, &input)
	}

	/// Clears the cache. removing all values.
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
}
