//! This crate implements an easy way to cache values for
//! a function. If you have a slow running function, this
//! can be used to speed up successive runs dramatically.
//! It is also quite useful for memoization of recursive
//! functions, to prevent calculating the same function
//! twice in different calls.
//!
//! Of particular note, this caching is done without
//! cloning or copying, allowing functions to return
//! large objects, while the cache only returns a reference
//! to them instead of copying them.
//!
//! # Allowed functions
//! This crate attempts to remain fairly flexible with
//! the functions it accepts. All of the following should
//! be allowed:
//!   * [`fn`][fn primitive] types.
//!   * [`Fn`] types that have no references.
//!   * [`Fn`] + 'static types that take only static references.
//!   * [`Fn`] + 'a types that take references of lifetime 'a.
//!
//! For obvious reasons, [`FnMut`] and [`FnOnce`] are not allowed,
//! as functions need to be rerunnable and pure.
//!
//! # Examples
//!
//! The caches can handle recursive functions, with a shortcut
//! for defining it with non-recursive functions. Each cache
//! has a `recursive` function to create a recursion capable
//! cache, but requires the function to accept the cache as the
//! first argument. Each `new` function takes a function that
//! does not require the cache as an argument.
//!
//! ## Non-recursive
//!
//! Here is an example for a function that takes a while to
//! calculate. Instead of running the calculations each time
//! you'd like to do it just once, and recall the value. The
//! results are stored in a [`HashCache`] for random access.
//!
//! ```rust
//! use fn_cache::{FnCache, HashCache};
//! use std::{thread, time};
//!
//! let sleep_time = time::Duration::from_secs(3);
//!
//! let mut cache = HashCache::new(|&x| {
//!     thread::sleep(sleep_time);
//!     x
//! });
//!
//! let start = time::Instant::now();
//! assert_eq!(cache.get(100), &100);
//! assert_eq!(cache.get(100), &100);
//!
//! // time elapsed is only slightly longer than the sleep time
//! // far less than twice.
//! assert!(time::Instant::now() - start < sleep_time.mul_f32(1.1));
//! ```
//!
//! ## Recursive
//!
//! The following example shows a recursive fibonacci
//! implementation, which would be O(2ⁿ) without
//! memoization (caching). With memoization, it becomes
//! O(n), and can easily be calculated.
//!
//! ```rust
//! use fn_cache::{FnCache, HashCache};
//!
//! let mut cache = HashCache::<u8,u128>::recursive(|cache, x|
//!     match x {
//!         0 => 0,
//!         1 => 1,
//!         _ => *cache.get(x - 1) + *cache.get(x - 2),
//!     }
//! );
//!
//! assert_eq!(
//!     *cache.get(186),
//!     332_825_110_087_067_562_321_196_029_789_634_457_848
//! );
//! ```
//!
//! For even bigger results, the [num] crate might be employed.
//! In order to avoid copying the `BigUint`s while accessing the
//! cache twice, using [`FnCacheMany::get_many`] can be used to get
//! multiple values at once, to avoid trying to take a reference and
//! then mutating again. Additionally, since the inputs start at 0
//! and each value must be filled before the next is calculated, you
//! might use a [`VecCache`] as an optimization.
//!
//! ```rust
//! use fn_cache::{FnCache, FnCacheMany, VecCache};
//! use num_bigint::BigUint;
//!
//! let mut cache = VecCache::recursive(|cache, x|
//!     match x {
//!         0 => BigUint::new(vec![0]),
//!         1 => BigUint::new(vec![1]),
//!         _ => cache.get_many([x - 1, x - 2]).into_iter().sum(),
//!     }
//! );
//!
//! assert_eq!(
//!     cache.get(999),
//!     &BigUint::parse_bytes(b"26863810024485359386146727202142923967616609318986952340123175997617981700247881689338369654483356564191827856161443356312976673642210350324634850410377680367334151172899169723197082763985615764450078474174626", 10).unwrap()
//! );
//! ```
//!
//! [fn primitive]: https://doc.rust-lang.org/std/primitive.fn.html
//! [`Rc`]: std::rc::Rc
//! [num]: https://docs.rs/num/
mod btree_cache;
mod cache;
mod fn_cache;
mod hash_cache;
mod raw_cache;
mod tests;
mod vec_cache;

pub use crate::btree_cache::BTreeCache;
pub use crate::raw_cache::RawCache;
pub use crate::fn_cache::{FnCache, FnCacheMany};
pub use crate::hash_cache::HashCache;
pub use crate::vec_cache::VecCache;
