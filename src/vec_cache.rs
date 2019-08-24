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
///
/// The value in the cache `V` can be different than
/// the output of the function `O`, as long as
/// `O` implements `Into<V>`. If no conversion is
/// required, than the `V` parameter can be elided.
pub struct VecCache<'a,O,V=O>
where
	O: Into<V>,
{
	pub(crate) cache: Vec<V>,
	f: *mut (dyn Fn(&mut Self, usize) -> O + 'a),
}

impl<'a,O,V> VecCache<'a,O,V>
where
	O: Into<V>,
{
	/// Create a cache for the provided function. If the
	/// function stores references, the cache can only
	/// live as long as those references.
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&mut Self, usize) -> O + 'a
	{
		VecCache {
			cache: Vec::default(),
			f: Box::into_raw(Box::from(f)),
		}
	}

	/// Retrieve a value stored in the cache. If the
	/// value does not yet exist in the cache, the
	/// function is called, and the result is added
	/// to the cache before returning it.
	pub fn get(&mut self, input: usize) -> &V {
		let len = self.cache.len();

		if len <= input {
			self.cache.reserve(input - len + 1);
		}

		while self.cache.len() <= input {
			let next = self.cache.len();
			let next_val = self.function(next).into();
			self.cache.push(next_val);
		}

		self.cache.get(input).unwrap()
	}

	fn function(&mut self, input: usize) -> V {
		unsafe { (*self.f)(self, input).into() }
	}
}

#[doc(hidden)]
impl<'a,O,V> Drop for VecCache<'a,O,V>
where
	O: Into<V>,
{
	fn drop(&mut self) {
		#[allow(unused_must_use)]
		unsafe { Box::from_raw(self.f); }
	}
}
