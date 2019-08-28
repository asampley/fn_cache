use std::collections::HashMap;

use core::cmp::Eq;
use core::hash::Hash;
use core::ops::Index;

/// A cache for a function which uses a `HashMap`.
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
/// The requirements for a `HashMap` must be met,
/// specifically the keys must implement `Eq` and
/// `Hash`, and the following propery must hold:
///
/// ```k1 == k2 -> hash(k1) == hash(k2)```
pub struct HashCache<'a,I,O,V=O>
where
	I: Eq + Hash,
{
	pub(crate) cache: HashMap<I,V>,
	f: *mut (dyn Fn(&mut Self, &I) -> O + 'a),
}

impl<'a,I,O,V> HashCache<'a,I,O,V>
where
	I: Eq + Hash,
	O: Into<V>,
{
	/// Create a cache for the provided function. If the
	/// function stores references, the cache can only
	/// live as long as those references.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&mut Self, &I) -> O + 'a
	{
		HashCache {
			cache: HashMap::default(),
			f: Box::into_raw(Box::new(f)),
		}
	}

	/// Retrieve a value stored in the cache. If the
	/// value does not yet exist in the cache, the
	/// function is called, and the result is added
	/// to the cache before returning it.
	pub fn get(&mut self, input: I) -> &V {
		if self.cache.contains_key(&input) {
			self.cache.index(&input)
		} else {
			let output = self.compute(&input);
			self.cache.entry(input).or_insert(output.into())
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
}

#[doc(hidden)]
impl<'a,I,O,V> Drop for HashCache<'a,I,O,V>
where
	I: Eq + Hash,
{
	fn drop(&mut self) {
		#[allow(unused_must_use)]
		unsafe { Box::from_raw(self.f); }
	}
}
