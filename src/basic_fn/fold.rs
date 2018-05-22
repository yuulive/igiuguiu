// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Used for fold the double end iterator from the beginning with init value and fold function
/// 
/// # Arguments
/// 
/// * `init`: initial point of folding, must be same type with final result
/// * `f`: f :: (R, T) -> R, fold function
/// * `it`: DoubleEndedIterator T
pub fn foldl<T,R>(init: R, f: impl Fn(R,T) -> R, it: impl DoubleEndedIterator<Item=T>) -> R {
    it.fold(init, f)
}

/// Curry macro of [`foldl`]
/// 
/// **Signature**: foldl :: R -> (R -> T -> R) -> DoubleEndedIterator T -> R
#[macro_export]
macro_rules! foldl {
    ($init:expr,$f:expr) => {
        move |it| foldl($init,$f,it)
    };
    ($init:expr) => {
        move |f,it| foldl($init,f,it)
    };
}

/// Used for fold the double end iterator from the end with init value and fold function
/// 
/// # Arguments
/// 
/// * `init`: initial point of folding, must be same type with final result
/// * `f`: f :: (R, T) -> R, fold function
/// * `it`: DoubleEndedIterator T
pub fn foldr<T,R>(init: R, f: impl Fn(R,T) -> R, it: impl DoubleEndedIterator<Item=T>) -> R {  
    it.rev().fold(init, f)
}

/// Curry macro of [`foldr`]
/// 
/// **Signature**: foldr :: R -> (R -> T -> R) -> DoubleEndedIterator T -> R
#[macro_export]
macro_rules! foldr {
    ($init:expr,$f:expr) => {
        move |it| foldr($init,$f,it)
    };
    ($init:expr) => {
        move |f,it| foldr($init,f,it)
    }
}