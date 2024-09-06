use crate::container::{
	ContainerClear, ContainerLen, ContainerRemove, ContainerReserve, SparseContainer,
};
use crate::{FnCache, FnCacheMany};

/// A generic cache for a function backed by anything that implements the [`SparseContainer`]
/// trait.
///
/// Not all containers may support the precise access pattern, such as [`VecCache`] which
/// implements [`FnCache`] directly, but most uses can simply implement [`SparseContainer`] for
/// their container and immediately have everything working with [`GenericCache`].
///
/// The cache takes ownership of all inputs, but only passes a reference to the function, allowing
/// it to store the input in the cache without any copies or clones. Additionally, the function is
/// shared using by using a [`RefCache`] when actually calling the function, preventing any
/// reference counting or clones of the closure.
pub struct GenericCache<'f, C: SparseContainer> {
	pub(crate) cache: C,
	f: Box<dyn Fn(&mut RefCache<C>, &C::Input) -> C::Output + Send + 'f>,
}

impl<'f, C: SparseContainer> GenericCache<'f, C> {
	/// Create a `GenericCache` out of a cache and a function.
	///
	/// Using this function you can pre-initialize some values into the cache if desired, change
	/// settings using specific constructors on the cache type, or any variation. If a default
	/// version of the cache is sufficient for your needs, [`Self::new`] may be less verbose.
	///
	/// ```
	/// # use fn_cache::GenericCache;
	/// # use std::collections::HashMap;
	/// let cache = GenericCache::with_cache(HashMap::<usize, usize>::new(), |x: &usize| *x);
	/// ```
	pub fn with_cache(cache: C, f: impl Fn(&C::Input) -> C::Output + Send + 'f) -> Self {
		Self {
			cache,
			f: Box::new(move |_, i| f(i)),
		}
	}

	/// Create a `GenericCache` out of a cache and a recursive function.
	///
	/// Using this function you can pre-initialize some values into the cache if desired, change
	/// settings using specific constructors on the cache type, or any variation. If a default
	/// version of the cache is sufficient for your needs, [`Self::recursive`] may be less verbose.
	///
	/// ```
	/// # use fn_cache::{FnCacheMany, GenericCache};
	/// # use std::collections::HashMap;
	/// let cache = GenericCache::recursive_with_cache(HashMap::<usize, usize>::new(), |cache, x| match x {
	///     0 => 1,
	///     1 => 1,
	///     _ => cache.get_many([x - 1, x - 2]).into_iter().sum()
	/// });
	/// ```
	pub fn recursive_with_cache(
		cache: C,
		f: impl Fn(&mut RefCache<C>, &C::Input) -> C::Output + Send + 'f,
	) -> Self {
		Self {
			cache,
			f: Box::new(f),
		}
	}

	/// Get a reference to the underlying cache object, letting you use functions exclusive to the
	/// cache type (as long they only need `&self` of course).
	pub fn cache(&self) -> &C {
		&self.cache
	}
}

impl<'f, C> GenericCache<'f, C>
where
	C: SparseContainer + Default,
{
	/// Create a `GenericCache` using the `Default` implementation of the [`Cache`] type.
	///
	/// If a specific instance of a cache is required, see [`Self::with_cache`].
	///
	/// ```
	/// # use fn_cache::GenericCache;
	/// # use std::collections::HashMap;
	/// let cache: GenericCache<HashMap<_,_>> = GenericCache::new(|x: &usize| *x);
	/// ```
	pub fn new(f: impl Fn(&C::Input) -> C::Output + Send + 'f) -> Self {
		Self::with_cache(Default::default(), f)
	}

	/// Create a `GenericCache` using the `Default` implementation of the [`Cache`] type, using a
	/// recursive function.
	///
	/// If a specific instance of a cache is required, see [`Self::recursive_with_cache`].
	///
	/// ```
	/// # use fn_cache::{FnCacheMany, GenericCache};
	/// # use std::collections::HashMap;
	/// let cache: GenericCache<HashMap<usize, u64>> = GenericCache::recursive(|cache, x| match x {
	///     0 => 1,
	///     1 => 1,
	///     _ => cache.get_many([x - 1, x - 2]).into_iter().sum()
	/// });
	/// ```
	///
	/// # Issues
	/// Currently it does not work if you pass in a recursive function generic over [`FnCache`] via
	/// a function pointer. Wrap the pointer in a closure.
	///
	/// ```
	/// # use fn_cache::{FnCache, GenericCache};
	/// # use std::collections::HashMap;
	/// fn increment(cache: &mut impl FnCache<usize, usize>, x: &usize) -> usize {
	///     match x {
	///         0 => 0,
	///         _ => cache.get(x - 1) + 1,
	///     }
	/// }
	///
	/// // no good
	/// //let cache: GenericCache<HashMap<_, _>> = GenericCache::recursive(increment);
	/// //okay
	/// let cache: GenericCache<HashMap<_, _>> = GenericCache::recursive(|c, i| increment(c, i));
	/// ```
	pub fn recursive(f: impl Fn(&mut RefCache<C>, &C::Input) -> C::Output + Send + 'f) -> Self {
		Self::recursive_with_cache(Default::default(), f)
	}
}

impl<'f, C: SparseContainer + ContainerLen> GenericCache<'f, C> {
	/// Returns the number of elements in the cache.
	pub fn len(&self) -> usize {
		self.cache.len()
	}
}

impl<'f, C: SparseContainer + ContainerClear> GenericCache<'f, C> {
	/// Clears the cache, removing all key-value pairs.
	/// Keeps the allocated memory for reuse.
	pub fn clear(&mut self) {
		self.cache.clear()
	}
}

impl<'f, C: SparseContainer + ContainerReserve> GenericCache<'f, C> {
	/// Reserves capacity for at least `additional` more elements
	/// to be inserted in the cache. The collection may
	/// reserve more space to avoid frequent reallocations.
	pub fn reserve(&mut self, additional: usize) {
		self.cache.reserve(additional)
	}
}

impl<'f, C: ContainerRemove> GenericCache<'f, C> {
	/// Removes the input from the cache, returning any value
	/// if the input was previously in the cache.
	pub fn remove(&mut self, input: &C::Input) -> Option<C::Output> {
		self.cache.remove(input)
	}
}

impl<'f, C: SparseContainer> FnCache<C::Input, C::Output> for GenericCache<'f, C> {
	fn get(&mut self, input: C::Input) -> &C::Output {
		if self.cache.has(&input) {
			self.cache.get(&input).unwrap()
		} else {
			let mut ref_cache = RefCache::new(&mut self.cache, self.f.as_ref());
			let output = (self.f)(&mut ref_cache, &input);
			self.cache.put(input, output)
		}
	}
}

impl<'f, C> FnCacheMany<C::Input, C::Output> for GenericCache<'f, C>
where
	C: SparseContainer,
	C::Input: Clone,
{
	fn get_many<const N: usize>(&mut self, inputs: [C::Input; N]) -> [&C::Output; N] {
		for i in &inputs {
			self.get(i.clone());
		}

		inputs.map(|i| self.cache.get(&i).unwrap())
	}
}

pub struct RefCache<'c, C: SparseContainer> {
	pub(crate) cache: &'c mut C,
	f: &'c (dyn Fn(&mut Self, &C::Input) -> C::Output + Send),
}

impl<'c, C: SparseContainer> RefCache<'c, C> {
	pub fn new(
		cache: &'c mut C,
		f: &'c (dyn Fn(&mut Self, &C::Input) -> C::Output + Send),
	) -> Self {
		Self { cache, f }
	}
}

impl<'c, C> FnCache<C::Input, C::Output> for RefCache<'c, C>
where
	C: SparseContainer,
{
	fn get(&mut self, input: C::Input) -> &C::Output {
		if self.cache.has(&input) {
			self.cache.get(&input).unwrap()
		} else {
			let output = (self.f)(self, &input);
			self.cache.put(input, output)
		}
	}
}

impl<'c, C> FnCacheMany<C::Input, C::Output> for RefCache<'c, C>
where
	C: SparseContainer,
	C::Input: Clone,
{
	fn get_many<const N: usize>(&mut self, inputs: [C::Input; N]) -> [&C::Output; N] {
		for i in &inputs {
			self.get(i.clone());
		}

		inputs.map(|i| self.cache.get(&i).unwrap())
	}
}
