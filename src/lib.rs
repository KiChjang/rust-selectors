/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![cfg_attr(feature = "heap_size", feature(plugin, custom_derive))]
#![cfg_attr(feature = "heap_size", plugin(heapsize_plugin))]

#[cfg(feature = "heap_size")] extern crate heapsize;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate cssparser;
#[macro_use] extern crate matches;
extern crate fnv;

pub mod bloom;
pub mod matching;
pub mod parser;
mod tree;

pub use parser::{SelectorImpl, Parser, SelectorList};
pub use tree::Element;
pub use tree::{MatchAttr, MatchAttrGeneric};
