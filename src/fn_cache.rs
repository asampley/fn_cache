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
