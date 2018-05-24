// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! KaguyaRs aims to provide a common functional programming lib on Rust.
//! 
//! Basic functions and ADTs with implementation will be provided as much as I can.

pub mod basic_fn {
    pub mod pipe;
    pub mod compose;
    pub mod map;
    pub mod filter;
    pub mod fold;
    pub mod sum;
    pub mod list;
}