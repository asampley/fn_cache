pub trait FnCache<I,O> {
	/// Retrieve a value stored in the cache. If the
	/// value does not yet exist in the cache, the
	/// function is called, and the result is added
	/// to the cache before returning it.
	fn get(&mut self, input: I) -> &O;
}
