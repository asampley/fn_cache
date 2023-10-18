/// The generic trait for all caches.
///
/// This trait is implemented on all caches. This allows
/// someone to write a function like
/// `fn f(cache: &mut impl FnCache<u32,u32>, x: &u32) -> u32`
/// and have it work for all the caches written in this crate.
pub trait FnCache<I, O> {
	/// Retrieve a value stored in the cache. If the
	/// value does not yet exist in the cache, the
	/// function is called, and the result is added
	/// to the cache before returning it.
	fn get(&mut self, input: I) -> &O;
}

/// The generic trait for caches which support getting multiple
/// values.
///
/// This trait may have additional restrictions such as I is
/// [`Clone`]. This allows someone to write a function like
/// `fn f(cache: &mut impl FnCacheMany<u32,u32>, x: &u32) -> u32` and
/// have it work in most cases when the key is cloneable with caches
/// in this crate.
pub trait FnCacheMany<I, O>: FnCache<I, O> {
	/// Retrieve multiple values stored in the cache.
	/// If any of the values do not yet exist, the
	/// function is called, and the result is added
	/// to the cache before returning them.
	///
	/// This is helpful for cases which may require a
	/// recursive definition that uses multiple values
	/// at once.
	fn get_many<const N: usize>(&mut self, inputs: [I; N]) -> [&O; N];
}
