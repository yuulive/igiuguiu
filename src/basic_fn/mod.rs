// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Module basic_fn provide basic short-hand function for data operations.
//! This include function and macros. For details please refer [Rust Docs](https://docs.rs/)

#[macro_use]
pub mod pipe;
#[macro_use]
pub mod compose;
#[macro_use]
pub mod map;
#[macro_use]
pub mod filter;
#[macro_use]
pub mod fold;