pub trait Cache: Sized {
	type Input;
	type Output;

	fn has(&self, input: &Self::Input) -> bool;
	fn get(&self, input: &Self::Input) -> Option<&Self::Output>;
	fn put(&mut self, input: Self::Input, output: Self::Output) -> &Self::Output;
}

pub trait CacheLen: Cache {
	fn len(&self) -> usize;
}

pub trait CacheClear: Cache {
	fn clear(&mut self);
}

pub trait CacheReserve: Cache {
	fn reserve(&mut self, additional: usize);
}

pub trait CacheRemove: Cache {
	fn remove(&mut self, input: &Self::Input) -> Option<Self::Output>;
}

impl<I, O, S> Cache for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	type Input = I;
	type Output = O;

	fn has(&self, input: &I) -> bool {
		self.contains_key(input)
	}
	fn get(&self, input: &I) -> Option<&O> {
		self.get(input)
	}

	fn put(&mut self, input: I, output: O) -> &O {
		self.entry(input).or_insert(output)
	}
}

impl <I, O, S> CacheLen for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
    fn len(&self) -> usize {
        self.len()
    }
}

impl <I, O, S> CacheClear for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	fn clear(&mut self) { self.clear() }
}

impl <I, O, S> CacheReserve for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	fn reserve(&mut self, additional: usize) { self.reserve(additional) }
}

impl <I, O, S> CacheRemove for std::collections::HashMap<I, O, S>
where
	I: Eq + std::hash::Hash,
	S: std::hash::BuildHasher,
{
	fn remove(&mut self, input: &I) -> Option<O> { self.remove(input) }
}
