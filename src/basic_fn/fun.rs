// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter::Sum;

/// Used for data projection via mapping function.
/// 
/// # Arguments
/// 
/// * `f`: f :: T -> U
/// * `it`: [`Iterator`] T
pub fn map<T,U>(f: impl Fn(T) -> U, it: impl Iterator<Item=T>) -> impl Iterator<Item=U> {
    it.map(f)
}

/// Used for sum [`Iterator`]<T>
/// 
/// # Arguments
/// 
/// * `it`: [`Iterator`] T
pub fn sum<T: Sum>(it: impl Iterator<Item=T>) -> T {
    it.sum()
}

/// Used for fold the double end iterator from the beginning with init value and fold function
/// 
/// # Arguments
/// 
/// * `init`: initial point of folding, must be same type with final result
/// * `f`: f :: (R, T) -> R, fold function
/// * `it`: [`DoubleEndedIterator`] T
pub fn foldl<T,R>(init: R, f: impl Fn(R,T) -> R, it: impl DoubleEndedIterator<Item=T>) -> R {
    it.fold(init, f)
}

/// Used for fold the double end iterator from the end with init value and fold function
/// 
/// # Arguments
/// 
/// * `init`: initial point of folding, must be same type with final result
/// * `f`: f :: (R, T) -> R, fold function
/// * `it`: [`DoubleEndedIterator`] T
pub fn foldr<T,R>(init: R, f: impl Fn(R,T) -> R, it: impl DoubleEndedIterator<Item=T>) -> R {  
    it.rev().fold(init, f)
}

/// Used for filter [`Iterator`]<T>
/// 
/// # Arguments
/// 
/// * `f`: f :: T -> [`bool`], function to filter item
/// * `it`: [`Iterator`] to be filtered
pub fn filter<T>(f: impl Fn(&T) -> bool, it: impl Iterator<Item=T>) -> impl Iterator<Item=T> {
    it.filter(f)
}

/// Used for reverse filter [`Iterator`]<T>
/// 
/// # Arguments
/// 
/// * `f`: f :: T -> [`bool`], function to reverse filter item
/// * `it`: [`Iterator`] to be filtered
pub fn filter_not<T>(f: impl Fn(&T) -> bool, it: impl Iterator<Item=T>) -> impl Iterator<Item=T> {
    it.filter(move |x| !f(x))
}

/// Getting the first element of [`Iterator`]<T>
///
/// # Arguments
///
/// * `it`: [`Iterator`] T
pub fn head<T>(mut it: impl Iterator<Item=T>) -> Option<T> {
    it.next()
}

/// Getting all elements of [`Iterator`]<T> except first as [`Vec`]<T>
///
/// # Arguments
///
/// * `it`: [`Iterator`] T
pub fn tail<T>(mut it: impl Iterator<Item=T>) -> Option<Vec<T>> {
    if let Some(_) = it.next() {
        let mut ret = Vec::new();
        ret.extend(it);
        return Some(ret);
    }
    None
}

/// Getting last element of [`Iterator`]<T>
///
/// # Arguments
///
/// * `it`: [`Iterator`] T
pub fn last<T>(mut it: impl Iterator<Item=T>) -> Option<T> {
    let mut ret = None;
    for i in it {
        ret = Some(i);
    }
    ret
}

/// Getting all elements of [`Iterator`]<T> except the last one
///
/// # Arguments
///
/// * `it`: [`Iterator`] T
pub fn init<T>(mut it: impl Iterator<Item=T>) -> Option<Vec<T>> {
    let mut ret = Vec::new();
    ret.extend(it);
    let size = ret.len();
    if size == 0 {
        return None;
    }
    ret.remove(size-1);
    Some(ret)
}

/// Skip first n elements and return a new [`Vec`].
///
/// # Arguments
///
/// * `n`: elements count to skip
/// * `it`: [`Iterator`] T
pub fn skip<T>(n: usize, mut it: impl Iterator<Item=T>) -> Vec<T> {
    for _ in 0..n {
        it.next();
    }
    let mut ret = Vec::new();
    ret.extend(it);
    ret
}

/// Take first n elements and return a new [`Vec`].
///
/// # Arguments
///
/// * `n`: elements count to take
/// * `it`: [`Iterator`] T
pub fn take<T>(n: usize, mut it: impl Iterator<Item=T>) -> Vec<T> {
    let mut ret = Vec::new();
    for _ in 0..n {
        match it.next() {
            None => break,
            Some(item) => ret.push(item)
        }
    }
    ret
}