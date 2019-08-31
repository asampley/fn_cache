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
//!   * [fn][fn primitive] types.
//!   * [Fn] types that have no references.
//!   * [Fn] + 'static types that take only static references.
//!   * [Fn] + 'a types that take references of lifetime 'a.
//!
//! For obvious reasons, [FnMut] and [FnOnce] are not allowed,
//! as functions need to be rerunnable and pure.
//!
//! # Examples
//! The following example shows a recursive fibonacci
//! implementation, which would be O(2ⁿ) without
//! memoization (caching). With memoization, it becomes
//! O(n), and can easily be calculated.
//!
//! ```rust
//! use fn_cache::{FnCache, HashCache};
//!
//! let mut cache = HashCache::<u8,u128>::new(|cache, x|
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
//! cache twice, you can to change the result to be stored in an
//! [Rc]. This can be done without changing the function return
//! type, because [HashCache] can convert the result of the
//! function itself.
//!
//! ```rust
//! use std::rc::Rc;
//! use fn_cache::{FnCache, HashCache};
//! use num_bigint::BigUint;
//!
//! let mut cache = HashCache::<u64,BigUint,Rc<BigUint>>::new(|cache, x|
//!     match x {
//!         0 => BigUint::new(vec![0]),
//!         1 => BigUint::new(vec![1]),
//!         _ => cache.get(x - 1).clone().as_ref()
//!             + cache.get(x - 2).clone().as_ref(),
//!     }
//! );
//!
//! assert_eq!(
//!     cache.get(999).clone().as_ref(),
//!     &BigUint::parse_bytes(b"26863810024485359386146727202142923967616609318986952340123175997617981700247881689338369654483356564191827856161443356312976673642210350324634850410377680367334151172899169723197082763985615764450078474174626", 10).unwrap()
//! );
//! ```
//!
//! [fn primitive]: https://doc.rust-lang.org/std/primitive.fn.html
//! [Fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
//! [FnMut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
//! [FnOnce]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
//! [HashCache]: struct.HashCache.html
//! [Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
//! [num]: https://docs.rs/num/
mod fn_cache;
mod hash_cache;
mod vec_cache;
mod btree_cache;
mod tests;

pub use crate::fn_cache::FnCache;
pub use crate::hash_cache::HashCache;
pub use crate::vec_cache::VecCache;
pub use crate::btree_cache::BTreeCache;
