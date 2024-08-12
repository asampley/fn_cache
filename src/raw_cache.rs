use crate::cache::{Cache, CacheClear, CacheLen, CacheRemove, CacheReserve};
use crate::{FnCache, FnCacheMany};

pub trait FnSimple<C>: Fn(&C::Input) -> C::Output + Send
where
	C: Cache,
{
}
pub trait FnRecursive<C>:
	Fn(&mut RefCache<C>, &C::Input) -> C::Output + Send
where
	C: Cache,
{
}

impl<T, C> FnSimple<C> for T
where
	T: Fn(&C::Input) -> C::Output + Send,
	C: Cache,
{
}

impl<T, C> FnRecursive<C> for T
where
	T: Fn(&mut RefCache<C>, &C::Input) -> C::Output + Send,
	C: Cache,
{
}

pub struct RawCache<'f, C: Cache> {
	pub(crate) cache: C,
	f: Box<dyn FnRecursive<C> + 'f>,
}

impl<'f, C: Cache> RawCache<'f, C> {
	pub fn with_cache(cache: C, f: impl FnSimple<C> + 'f) -> Self {
		Self {
			cache,
			f: Box::new(move |_, i| f(i)),
		}
	}

	pub fn recursive_with_cache(cache: C, f: impl FnRecursive<C> + 'f) -> Self {
		Self {
			cache,
			f: Box::new(f),
		}
	}

	fn as_ref(&mut self) -> RefCache<'_, C> {
		RefCache {
			cache: &mut self.cache,
			f: &self.f,
		}
	}
}

impl<'f, C> RawCache<'f, C>
where
	C: Cache + Default,
{
	pub fn new(f: impl FnSimple<C> + 'f) -> Self {
		Self::with_cache(Default::default(), f)
	}

	pub fn recursive(f: impl FnRecursive<C> + 'f) -> Self {
		Self::recursive_with_cache(Default::default(), f)
	}
}

impl<'f, C: Cache, I, O> FnCache<C::Input, C::Output> for RawCache<'f, C>
where
	C: Cache<Input = I, Output = O>,
{
	fn get(&mut self, input: I) -> &O {
		if self.cache.has(&input) {
			self.cache.get(&input).unwrap()
		} else {
			let mut ref_cache = self.as_ref();
			let output = (ref_cache.f)(&mut ref_cache, &input);
			self.cache.put(input, output)
		}
	}
}

impl<'f, C> FnCacheMany<C::Input, C::Output> for RawCache<'f, C>
where
	C: Cache,
	C::Input: Clone,
{
	fn get_many<const N: usize>(&mut self, inputs: [C::Input; N]) -> [&C::Output; N] {
		for i in &inputs {
			self.get(i.clone());
		}

		inputs.map(|i| self.cache.get(&i).unwrap())
	}
}

impl<'f, C: CacheLen> RawCache<'f, C> {
	pub fn len(&self) -> usize {
		self.cache.len()
	}
}

impl<'f, C: CacheClear> RawCache<'f, C> {
	pub fn clear(&mut self) {
		self.cache.clear()
	}
}

impl<'f, C: CacheReserve> RawCache<'f, C> {
	pub fn reserve(&mut self, additional: usize) {
		self.cache.reserve(additional)
	}
}

impl<'f, C: CacheRemove> RawCache<'f, C> {
	pub fn remove(&mut self, input: &C::Input) -> Option<C::Output> {
		self.cache.remove(input)
	}
}

pub struct RefCache<'c, C: Cache> {
	pub(crate) cache: &'c mut C,
	f: &'c (dyn Fn(&mut Self, &C::Input) -> C::Output + Send + 'c),
}

impl<'c, C> FnCache<C::Input, C::Output> for RefCache<'c, C>
where
	C: Cache,
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
	C: Cache,
	C::Input: Clone,
{
	fn get_many<const N: usize>(&mut self, inputs: [C::Input; N]) -> [&C::Output; N] {
		for i in &inputs {
			self.get(i.clone());
		}

		inputs.map(|i| self.cache.get(&i).unwrap())
	}
}
