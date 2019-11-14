use std::collections::BTreeMap;

use core::cmp::Ord;
use core::marker::PhantomData;
use core::ops::Index;

use crate::FnCache;

/// A cache for a function which uses a `BTreeMap`.
///
/// The cache takes ownership of all inputs, but
/// only passes a reference to the function,
/// allowing it to store the input in the cache
/// without any copies or clones.
///
/// The value in the cache `V` can be different than
/// the output of the function `O`, as long as
/// `O` implements `Into<V>`. If no conversion is
/// required, than the `V` parameter can be elided.
///
/// The requirements for a `BTreeMap` must be met,
/// specifically the keys must implement `Ord`
pub struct BTreeCache<'f, I, O, V = O>
where
	I: Ord,
{
	pub(crate) cache: BTreeMap<I, V>,
	f: *mut (dyn Fn(&mut Self, &I) -> O + 'f),

	// tell dropck that we will drop the Boxed Fn
	_phantom: PhantomData<Box<dyn Fn(&mut Self, &I) -> O + 'f>>,
}

impl<'f, I, O, V> FnCache<I, V> for BTreeCache<'f, I, O, V>
where
	I: Ord,
	O: Into<V>,
{
	fn get(&mut self, input: I) -> &V {
		if self.cache.contains_key(&input) {
			self.cache.index(&input)
		} else {
			let output = self.compute(&input);
			self.cache.entry(input).or_insert(output.into())
		}
	}
}

impl<'f, I, O, V> BTreeCache<'f, I, O, V>
where
	I: Ord,
	O: Into<V>,
{
	/// Create a cache for the provided function. If the
	/// function stores references, the cache can only
	/// live as long as those references.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&mut Self, &I) -> O + 'f,
	{
		Self {
			cache: BTreeMap::default(),
			f: Box::into_raw(Box::new(f)),
			_phantom: Default::default(),
		}
	}

	fn compute(&mut self, input: &I) -> O {
		unsafe { (*self.f)(self, input) }
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
	pub fn remove(&mut self, input: &I) -> Option<V> {
		self.cache.remove(input)
	}
}

#[doc(hidden)]
impl<'f, I, O, V> Drop for BTreeCache<'f, I, O, V>
where
	I: Ord,
{
	fn drop(&mut self) {
		#[allow(unused_must_use)]
		unsafe {
			Box::from_raw(self.f);
		}
	}
}
