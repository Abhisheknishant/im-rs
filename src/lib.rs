// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # Immutable Data Structures for Rust
//!
//! This library implements several of the more commonly useful immutable data
//! structures for Rust.
//!
//! ## What are immutable data structures?
//!
//! Immutable data structures are data structures which can be copied and
//! modified efficiently without altering the original. The most uncomplicated
//! example of this is the venerable [cons list][cons-list]. This crate offers a
//! selection of more modern and flexible data structures with similar
//! properties, tuned for the needs of Rust developers.
//!
//! Briefly, the following data structures are provided:
//!
//! * [Vectors][vector::Vector] based on [RRB trees][rrb-tree]
//! * [Hash maps][hashmap::HashMap]/[sets][hashset::HashSet] based on [hash
//!   array mapped tries][hamt]
//! * [Ordered maps][ordmap::OrdMap]/[sets][ordset::OrdSet] based on
//!   [B-trees][b-tree]
//!
//! ## Why Would I Want This?
//!
//! While immutable data structures can be a game changer for other
//! programming languages, the most obvious benefit - avoiding the
//! accidental mutation of data - is already handled so well by Rust's
//! type system that it's just not something a Rust programmer needs
//! to worry about even when using data structures that would send a
//! conscientious Clojure programmer into a panic.
//!
//! Immutable data structures offer other benefits, though, some of
//! which are useful even in a language like Rust. The most prominent
//! is *structural sharing*, which means that if two data structures
//! are mostly copies of each other, most of the memory they take up
//! will be shared between them. This implies that making copies of an
//! immutable data structure is cheap: it's really only a matter of
//! copying a pointer and increasing a reference counter, where in the
//! case of [`Vec`][std::vec::Vec] you have to allocate the same
//! amount of memory all over again and make a copy of every element
//! it contains. For immutable data structures, extra memory isn't
//! allocated until you modify either the copy or the original, and
//! then only the memory needed to record the difference.
//!
//! Another goal of this library has been the idea that you shouldn't
//! even have to think about what data structure to use in any given
//! situation, until the point where you need to start worring about
//! optimisation - which, in practice, often never comes. Beyond the
//! shape of your data (ie. whether to use a list or a map), it should
//! be fine not to think too carefully about data structures - you can
//! just pick the one that has the right shape and it should have
//! acceptable performance characteristics for every operation you
//! might need. Specialised data structures will always be faster at
//! what they've been specialised for, but `im` aims to provide the
//! data structures which deliver the least chance of accidentally
//! using them for the wrong thing.
//!
//! For instance, [`Vec`][std::vec::Vec] beats everything at memory
//! usage, indexing and operations that happen at the back of the
//! list, but is terrible at insertion and removal, and gets worse the
//! closer to the front of the list you get.
//! [`VecDeque`][std::collections::VecDeque] adds a little bit of
//! complexity in order to make operations at the front as efficient
//! as operations at the back, but is still bad at insertion and
//! especially concatenation. [`Vector`][vector::Vector] adds another
//! bit of complexity, and could never match [`Vec`][std::vec::Vec] at
//! what it's best at, but in return every operation you can throw at
//! it can be completed in a reasonable amount of time - even normally
//! expensive operations like copying and especially concatenation are
//! reasonably cheap when using a [`Vector`][vector::Vector].
//!
//! It should be noted, however, that because of its simplicity,
//! [`Vec`][std::vec::Vec] actually beats [`Vector`][vector::Vector] even at its
//! strongest operations at small sizes, just because modern CPUs are
//! hyperoptimised for things like copying small chunks of contiguous memory -
//! you actually need to go past a certain size (usually in the vicinity of
//! several hundred elements) before you get to the point where
//! [`Vec`][std::vec::Vec] isn't always going to be the fastest choice.
//! [`Vector`][vector::Vector] attempts to overcome this by actually just being
//! an array at very small sizes, and being able to switch efficiently to the
//! full data structure when it grows large enough. Thus,
//! [`Vector`][vector::Vector] will actually be equivalent to
//! [Vec][std::vec::Vec] until it grows past the size of a single chunk.
//!
//! The maps - [`HashMap`][hashmap::HashMap] and
//! [`OrdMap`][ordmap::OrdMap] - generally perform similarly to their
//! equivalents in the standard library, but tend to run a bit slower
//! on the basic operations ([`HashMap`][hashmap::HashMap] is almost
//! neck and neck with its counterpart, while
//! [`OrdMap`][ordmap::OrdMap] currently tends to run 2-3x slower). On
//! the other hand, they offer the cheap copy and structural sharing
//! between copies that you'd expect from immutable data structures.
//!
//! In conclusion, the aim of this library is to provide a safe
//! default choice for the most common kinds of data structures,
//! allowing you to defer careful thinking about the right data
//! structure for the job until you need to start looking for
//! optimisations - and you may find, especially for larger data sets,
//! that immutable data structures are still the right choice.
//!
//! ## Data Structures
//!
//! We'll attempt to provide a comprehensive guide to the available
//! data structures below.
//!
//! ### Performance Notes
//!
//! "Big O notation" is the standard way of talking about the time
//! complexity of data structure operations. If you're not familiar
//! with big O notation, here's a quick cheat sheet:
//!
//! *O(1)* means an operation runs in constant time: it will take the
//! same time to complete regardless of the size of the data
//! structure.
//!
//! *O(n)* means an operation runs in linear time: if you double the
//! size of your data structure, the operation will take twice as long
//! to complete; if you quadruple the size, it will take four times as
//! long, etc.
//!
//! *O(log n)* means an operation runs in logarithmic time: for
//! *log<sub>2</sub>*, if you double the size of your data structure,
//! the operation will take one step longer to complete; if you
//! quadruple the size, it will need two steps more; and so on.
//! However, the data structures in this library generally run in
//! *log<sub>64</sub>* time, meaning you have to make your data
//! structure 64 times bigger to need one extra step, and 4096 times
//! bigger to need two steps. This means that, while they still count
//! as O(log n), operations on all but really large data sets will run
//! at near enough to O(1) that you won't usually notice.
//!
//! *O(n log n)* is the most expensive operation you'll see in this
//! library: it means that for every one of the *n* elements in your
//! data structure, you have to perform *log n* operations. In our
//! case, as noted above, this is often close enough to O(n) that it's
//! not usually as bad as it sounds, but even O(n) isn't cheap and the
//! cost still increases logarithmically, if slowly, as the size of
//! your data increases. O(n log n) basically means "are you sure you
//! need to do this?"
//!
//! *O(1)** means 'amortised O(1),' which means that an operation
//! usually runs in constant time but will occasionally be more
//! expensive: for instance,
//! [`Vector::push_back`][vector::Vector::push_back], if called in
//! sequence, will be O(1) most of the time but every 64th time it
//! will be O(log n), as it fills up its tail chunk and needs to
//! insert it into the tree. Please note that the O(1) with the
//! asterisk attached is not a common notation; it's just a convention
//! I've used in these docs to save myself from having to type
//! 'amortised' everywhere.
//!
//! ### Lists
//!
//! Lists are sequences of single elements which maintain the order in
//! which you inserted them. The only list in this library is
//! [`Vector`][vector::Vector], which offers the best all round
//! performance characteristics: it's pretty good at everything, even
//! if there's always another kind of list that's better at something.
//!
//! | Type | Algorithm | Constraints | Order | Push | Pop | Split | Append | Lookup |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | [`Vector<A>`][vector::Vector] | [RRB tree][rrb-tree] | [`Clone`][std::clone::Clone] | insertion | O(1)* | O(1)* | O(log n) | O(log n) | O(log n) |
//!
//! ### Maps
//!
//! Maps are mappings of keys to values, where the most common read
//! operation is to find the value associated with a given key. Maps
//! may or may not have a defined order. Any given key can only occur
//! once inside a map, and setting a key to a different value will
//! overwrite the previous value.
//!
//! | Type | Algorithm | Key Constraints | Order | Insert | Remove | Lookup |
//! | --- | --- | --- | --- | --- | --- |
//! | [`HashMap<K, V>`][hashmap::HashMap] | [HAMT][hamt] | [`Clone`][std::clone::Clone] + [`Hash`][std::hash::Hash] + [`Eq`][std::cmp::Eq] | undefined | O(log n) | O(log n) | O(log n) |
//! | [`OrdMap<K, V>`][ordmap::OrdMap] | [B-tree][b-tree] | [`Clone`][std::clone::Clone] + [`Ord`][std::cmp::Ord] | sorted | O(log n) | O(log n) | O(log n) |
//!
//! ### Sets
//!
//! Sets are collections of unique values, and may or may not have a
//! defined order. Their crucial property is that any given value can
//! only exist once in a given set.
//!
//! | Type | Algorithm | Constraints | Order | Insert | Remove | Lookup |
//! | --- | --- | --- | --- | --- | --- |
//! | [`HashSet<A>`][hashset::HashSet] | [HAMT][hamt] | [`Clone`][std::clone::Clone] + [`Hash`][std::hash::Hash] + [`Eq`][std::cmp::Eq] | undefined | O(log n) | O(log n) | O(log n) |
//! | [`OrdSet<A>`][ordset::OrdSet] | [B-tree][b-tree] | [`Clone`][std::clone::Clone] + [`Ord`][std::cmp::Ord] | sorted | O(log n) | O(log n) | O(log n) |
//!
//! ## In-place Mutation
//!
//! All of these data structures support in-place copy-on-write
//! mutation, which means that if you're the sole user of a data
//! structure, you can update it in place without taking the
//! performance hit of making a copy of the data structure before
//! modifying it (this is about an order of magnitude faster than
//! immutable operations, almost as fast as
//! [`std::collections`][std::collections]'s mutable data structures).
//!
//! Thanks to [`Rc`][std::rc::Rc]'s reference counting, we are able to
//! determine whether a node in a data structure is being shared with
//! other data structures, or whether it's safe to mutate it in place.
//! When it's shared, we'll automatically make a copy of the node
//! before modifying it. The consequence of this is that cloning a
//! data structure becomes a lazy operation: the initial clone is
//! instant, and as you modify the cloned data structure it will clone
//! chunks only where you change them, so that if you change the
//! entire thing you will eventually have performed a full clone.
//!
//! This also gives us a couple of other optimisations for free:
//! implementations of immutable data structures in other languages
//! often have the idea of local mutation, like Clojure's transients
//! or Haskell's `ST` monad - a managed scope where you can treat an
//! immutable data structure like a mutable one, gaining a
//! considerable amount of performance because you no longer need to
//! copy your changed nodes for every operation, just the first time
//! you hit a node that's sharing structure. In Rust, we don't need to
//! think about this kind of managed scope, it's all taken care of
//! behind the scenes because of our low level access to the garbage
//! collector (which, in our case, is just a simple
//! [`Rc`][std::rc::Rc]).
//!
//! ## Thread Safety
//!
//! The data structures in the `im` crate are thread safe, through
//! [`Arc`][std::sync::Arc]. This comes with a slight performance impact, so
//! that if you prioritise speed over thread safety, you may want to use the
//! `im-rc` crate instead, which is identical to `im` except that it uses
//! [`Rc`][std::rc::Rc] instead of [`Arc`][std::sync::Arc], implying that the
//! data structures in `im-rc` do not implement [`Send`][std::marker::Send] and
//! [`Sync`][std::marker::Sync]. This yields approximately a 20-25% increase in
//! general performance.
//!
//! ## Feature Flags
//!
//! `im` comes with optional support for the following crates through Cargo
//! feature flags. You can enable them in your `Cargo.toml` file like this:
//!
//! ```no_compile
//! [dependencies]
//! im = { version = "*", features = ["proptest", "serde"] }
//! ```
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | [`proptest`](https://crates.io/crates/proptest) | Strategies for all `im` datatypes under a `proptest` namespace, eg. `im::vector::proptest::vector()` |
//! | [`quickcheck`](https://crates.io/crates/quickcheck) | `Arbitrary` implementations for all `im` datatypes (not available in `im-rc`) |
//! | [`rayon`](https://crates.io/crates/rayon) | parallel iterator implementations for `Vector` (not available in `im-rc`) |
//! | [`serde`](https://crates.io/crates/serde) | `Serialize` and `Deserialize` implementations for all `im` datatypes |
//!
//! [std::collections]: https://doc.rust-lang.org/std/collections/index.html
//! [std::collections::VecDeque]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
//! [std::vec::Vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//! [std::rc::Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
//! [std::sync::Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [std::cmp::Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
//! [std::cmp::Ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
//! [std::clone::Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [std::clone::Clone::clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone
//! [std::hash::Hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
//! [std::marker::Send]: https://doc.rust-lang.org/std/marker/trait.Send.html
//! [std::marker::Sync]: https://doc.rust-lang.org/std/marker/trait.Sync.html
//! [hashmap::HashMap]: ./hashmap/struct.HashMap.html
//! [hashset::HashSet]: ./hashset/struct.HashSet.html
//! [ordmap::OrdMap]: ./ordmap/struct.OrdMap.html
//! [ordset::OrdSet]: ./ordset/struct.OrdSet.html
//! [vector::Vector]: ./vector/enum.Vector.html
//! [vector::Vector::push_back]: ./vector/enum.Vector.html#method.push_back
//! [rrb-tree]: https://infoscience.epfl.ch/record/213452/files/rrbvector.pdf
//! [hamt]: https://en.wikipedia.org/wiki/Hash_array_mapped_trie
//! [b-tree]: https://en.wikipedia.org/wiki/B-tree
//! [cons-list]: https://en.wikipedia.org/wiki/Cons#Lists

#![deny(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(has_specialisation, feature(specialization))]
#![cfg_attr(docs_rs_workaround, feature(slice_get_slice))]

extern crate typenum;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(feature = "quickcheck")]
extern crate quickcheck;

#[cfg(any(test, feature = "proptest"))]
#[macro_use]
extern crate proptest;

#[cfg(feature = "proptest")]
proptest!{}

#[cfg(any(test, feature = "serde"))]
extern crate serde;
#[cfg(test)]
extern crate serde_json;

#[cfg(all(threadsafe, any(test, feature = "rayon")))]
extern crate rayon;

mod config;
mod nodes;
mod sort;
mod sync;
mod util;

#[macro_use]
mod ord;
pub use ord::map as ordmap;
pub use ord::set as ordset;

#[macro_use]
mod hash;
pub use hash::map as hashmap;
pub use hash::set as hashset;

#[macro_use]
pub mod vector;

pub mod iter;

#[cfg(any(test, feature = "serde"))]
pub mod ser;

pub use nodes::sized_chunk as chunk;

pub use hashmap::HashMap;
pub use hashset::HashSet;
pub use ordmap::OrdMap;
pub use ordset::OrdSet;
pub use vector::Vector;

#[cfg(test)]
mod test;

/// Update a value inside multiple levels of data structures.
///
/// This macro takes a [`Vector`][Vector], [`OrdMap`][OrdMap] or [`HashMap`][HashMap],
/// a key or a series of keys, and a value, and returns the data structure with the
/// new value at the location described by the keys.
///
/// If one of the keys in the path doesn't exist, the macro will panic.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate im;
/// # use std::sync::Arc;
/// # fn main() {
/// let vec_inside_vec = vector![vector![1, 2, 3], vector![4, 5, 6]];
///
/// let expected = vector![vector![1, 2, 3], vector![4, 5, 1337]];
///
/// assert_eq!(expected, update_in![vec_inside_vec, 1 => 2, 1337]);
/// # }
/// ```
///
/// [Vector]: ../vector/enum.Vector.html
/// [HashMap]: ../hashmap/struct.HashMap.html
/// [OrdMap]: ../ordmap/struct.OrdMap.html
#[macro_export]
macro_rules! update_in {
    ($target:expr, $path:expr => $($tail:tt) => *, $value:expr ) => {{
        let inner = $target.get($path).expect("update_in! macro: key not found in target");
        $target.update($path, update_in!(inner, $($tail) => *, $value))
    }};

    ($target:expr, $path:expr, $value:expr) => {
        $target.update($path, $value)
    };
}

/// Get a value inside multiple levels of data structures.
///
/// This macro takes a [`Vector`][Vector], [`OrdMap`][OrdMap] or [`HashMap`][HashMap],
/// along with a key or a series of keys, and returns the value at the location inside
/// the data structure described by the key sequence, or `None` if any of the keys didn't
/// exist.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate im;
/// # use std::sync::Arc;
/// # fn main() {
/// let vec_inside_vec = vector![vector![1, 2, 3], vector![4, 5, 6]];
///
/// assert_eq!(Some(&6), get_in![vec_inside_vec, 1 => 2]);
/// # }
/// ```
///
/// [Vector]: ../vector/enum.Vector.html
/// [HashMap]: ../hashmap/struct.HashMap.html
/// [OrdMap]: ../ordmap/struct.OrdMap.html
#[macro_export]
macro_rules! get_in {
    ($target:expr, $path:expr => $($tail:tt) => * ) => {{
        $target.get($path).and_then(|v| get_in!(v, $($tail) => *))
    }};

    ($target:expr, $path:expr) => {
        $target.get($path)
    };
}

#[cfg(test)]
mod lib_test {
    #[test]
    fn update_in() {
        let vector = vector![1, 2, 3, 4, 5];
        assert_eq!(vector![1, 2, 23, 4, 5], update_in!(vector, 2, 23));
        let hashmap = hashmap![1 => 1, 2 => 2, 3 => 3];
        assert_eq!(
            hashmap![1 => 1, 2 => 23, 3 => 3],
            update_in!(hashmap, 2, 23)
        );
        let ordmap = ordmap![1 => 1, 2 => 2, 3 => 3];
        assert_eq!(ordmap![1 => 1, 2 => 23, 3 => 3], update_in!(ordmap, 2, 23));

        let vecs = vector![vector![1, 2, 3], vector![4, 5, 6], vector![7, 8, 9]];
        let vecs_target = vector![vector![1, 2, 3], vector![4, 5, 23], vector![7, 8, 9]];
        assert_eq!(vecs_target, update_in!(vecs, 1 => 2, 23));
    }

    #[test]
    fn get_in() {
        let vector = vector![1, 2, 3, 4, 5];
        assert_eq!(Some(&3), get_in!(vector, 2));
        let hashmap = hashmap![1 => 1, 2 => 2, 3 => 3];
        assert_eq!(Some(&2), get_in!(hashmap, &2));
        let ordmap = ordmap![1 => 1, 2 => 2, 3 => 3];
        assert_eq!(Some(&2), get_in!(ordmap, &2));

        let vecs = vector![vector![1, 2, 3], vector![4, 5, 6], vector![7, 8, 9]];
        assert_eq!(Some(&6), get_in!(vecs, 1 => 2));
    }
}
