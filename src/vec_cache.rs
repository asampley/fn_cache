use crate::FnCache;

/// A cache for a function which uses a `Vec`.
///
/// This cache is optimized for functions which must
/// be calculated in order, so that there can be no
/// gaps in the cache, and use `usize` as an argument.
///
/// If the function does not start at zero, or require
/// every previous value to be calculated for the next
/// one, consider using a [HashCache](struct.HashCache.html)
/// instead.
pub struct VecCache<'a, O>
{
	pub(crate) cache: Vec<O>,
	f: *mut (dyn Fn(&mut Self, &usize) -> O + 'a),
}

impl<'a, O> FnCache<usize, O> for VecCache<'a, O>
{
	fn get(&mut self, input: usize) -> &O {
		let len = self.cache.len();

		if len <= input {
			self.cache.reserve(input - len + 1);
		}

		while self.cache.len() <= input {
			let next = self.cache.len();
			let next_val = self.compute(next).into();
			self.cache.push(next_val);
		}

		self.cache.get(input).unwrap()
	}
}

impl<'a, O> VecCache<'a, O>
{
	/// Create a cache for the provided function. If the
	/// function stores references, the cache can only
	/// live as long as those references.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&mut Self, &usize) -> O + 'a,
	{
		VecCache {
			cache: Vec::default(),
			f: Box::into_raw(Box::from(f)),
		}
	}

	fn compute(&mut self, input: usize) -> O {
		unsafe { (*self.f)(self, &input).into() }
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

#[doc(hidden)]
impl<'a, O> Drop for VecCache<'a, O>
{
	fn drop(&mut self) {
		#[allow(unused_must_use)]
		unsafe {
			Box::from_raw(self.f);
		}
	}
}
