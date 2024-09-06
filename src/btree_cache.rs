use std::collections::BTreeMap;

use core::cmp::Ord;

use crate::{
	container::{ContainerClear, ContainerLen, ContainerRemove, SparseContainer},
	GenericCache,
};

/// A cache for a function which uses a [`BTreeMap`].
///
/// The cache takes ownership of all inputs, but
/// only passes a reference to the function,
/// allowing it to store the input in the cache
/// without any copies or clones.
///
/// The requirements for a [`BTreeMap`] must be met,
/// specifically the keys must implement [`Ord`]
pub type BTreeCache<'f, I, O> = GenericCache<'f, BTreeMap<I, O>>;

impl<I, O> SparseContainer for BTreeMap<I, O>
where
	I: Ord,
{
	type Input = I;
	type Output = O;

	fn has(&self, input: &Self::Input) -> bool {
		self.contains_key(input)
	}

	fn get(&self, input: &Self::Input) -> Option<&Self::Output> {
		self.get(input)
	}

	fn put(&mut self, input: Self::Input, output: Self::Output) -> &Self::Output {
		self.entry(input).or_insert(output)
	}
}

impl<I, O> ContainerLen for BTreeMap<I, O>
where
	I: Ord,
{
	fn len(&self) -> usize {
		self.len()
	}
}

impl<I, O> ContainerClear for BTreeMap<I, O>
where
	I: Ord,
{
	fn clear(&mut self) {
		self.clear()
	}
}

impl<I, O> ContainerRemove for BTreeMap<I, O>
where
	I: Ord,
{
	fn remove(&mut self, input: &Self::Input) -> Option<Self::Output> {
		self.remove(input)
	}
}
