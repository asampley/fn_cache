/// A generic trait for anything that would like to be used in a [`GenericCache`], allowing easy
/// extensibility using a container not covered by this library.
///
/// [`HashCache`] and [`BTreeCache`] are both just using `GenericCache` under the hood, by
/// implementing this trait on `HashMap` and `BTreeMap`.
///
/// If this trait doesn't quite fit with your container, you can also implement fully your own
/// [`FnCache`], which requires a bit more work than using this trait, but gives you full
/// generality. This is how [`VecCache`] is implemented, because it is not sparse, and must fill
/// all earlier indices.
pub trait SparseContainer: Sized {
	type Input;
	type Output;

	/// Returns true if the container is holding an output associated with `input`.
	fn has(&self, input: &Self::Input) -> bool {
		self.get(input).is_some()
	}

	/// Returns the output associated with `input`, if it exists.
	fn get(&self, input: &Self::Input) -> Option<&Self::Output>;

	/// Associate a new `output` with the key `input`, which can later be retrieved using
	/// [`Self::get`]
	fn put(&mut self, input: Self::Input, output: Self::Output) -> &Self::Output;
}

/// A trait to clear the container, for cases when caching may need to be temporary during some
/// calcuations, but may grow unbounded over the course of the program otherwise.
pub trait ContainerClear {
	/// Clears the cache, removing all key-value pairs.
	/// Keeps the allocated memory for reuse.
	fn clear(&mut self);
}

/// A trait to let you see how many values the container is holding.
pub trait ContainerLen {
	/// Returns the number of elements in the container.
	fn len(&self) -> usize;
}

/// A trait to reserve space in a container, in case you know how many values are about to enter
/// and can avoid reallocations by reserving more space at once.
pub trait ContainerReserve {
	/// Reserves capacity for at least `additional` more elements
	/// to be inserted in the cache. The collection may
	/// reserve more space to avoid frequent reallocations.
	fn reserve(&mut self, additional: usize);
}

/// A trait to remove items from a container, to prevent growth without bound.
pub trait ContainerRemove: SparseContainer {
	/// Removes the input from the cache, returning any value
	/// if the input was previously in the cache.
	fn remove(&mut self, input: &Self::Input) -> Option<Self::Output>;
}
