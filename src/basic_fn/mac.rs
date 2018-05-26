// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Curry macro of [map](basic_fn::fun::map)
/// 
/// **Signature**: map :: (T -> U) -> [`Iterator`] T -> [`Iterator`] U
#[macro_export]
macro_rules! map {
    ($f:expr) => {
        move |it| map($f, it)
    };
}

/// Shorthand macro of sum
/// 
/// Syntax:
/// 1. sum!(0;5) // equals sum(0..=5)
/// 2. sum!(0,1,2,3,4,5)
#[macro_export]
macro_rules! sum {
    ($i:expr;$j:expr) => {{
        sum($i..=$j)
    }};
    ($i:expr,$($j:expr),*) => {
        $i + sum!($($j),*)
    };
    ($i:expr) => {
        $i
    };
}

/// This macro is used to provide shortcut of function composition.
/// The order is last-in-last-invoke.
/// 
/// # Arguments
/// 
/// * `args` - function or closure separated by comma(,)
#[macro_export]
macro_rules! pipe  {
    ($($f:expr),*) => {
        |v| {pipe!(@NEXT v, $($f),*) }
    };

    (@NEXT $v:expr,$f:expr,$($rest:expr),*) => {
        pipe!(@NEXT $f($v), $($rest),*)
    };

    (@NEXT $v:expr,$last:expr) => {
        $last($v)
    };
}

/// This macro is used to provide ability of list comprehension.
/// Return [`Vec`]<T>.
/// 
/// Format:
/// `ls![{Mapper};{Iter}=>{Filterer}]`
/// 
/// Haskell form:
/// ```haskell
/// [{Mapper}(x) | x <- {Iter}, {Filterer}(x)]
/// ```
/// Python form:
/// ```python
/// [{Mapper}(x) for x in {Iter} if {Filterer}(x)]
/// ```
/// 
/// # Arguments
/// 
/// * `Mapper`: T -> U - Optional, function to map on item
/// * `Iter` - [`Iterator`]<T>
/// * `Filterer` T -> bool - Optional, to filter items
#[macro_export]
macro_rules! ls {
    ($it:expr) => {
        ls![|x| x;$it=>|_|true]
    };
    ($mapper:expr;$it:expr) => {
        ls![$mapper;$it=>|_| true]
    };
    ($it:expr=>$filterer:expr) => {
        ls![|x| x;$it=>$filterer]
    };
    ($mapper:expr;$it:expr=>$filterer:expr) => {{
        let mut ret = Vec::new();
        for i in $it {
            if $filterer(i) {
                ret.push($mapper(i));
            }
        }
        ret
    }};
}

/// Curry macro of [foldl](basic_fn::fun::foldl)
/// 
/// **Signature**: foldl :: R -> (R -> T -> R) -> [`DoubleEndedIterator`] T -> R
#[macro_export]
macro_rules! foldl {
    ($init:expr,$f:expr) => {
        move |it| foldl($init,$f,it)
    };
    ($init:expr) => {
        move |f,it| foldl($init,f,it)
    };
    ($init:expr=>) => {
        move |f| (move |it| foldl($init,f,it))
    };
}

/// Curry macro of [foldr](basic_fn::fun::foldr)
/// 
/// **Signature**: foldr :: R -> (R -> T -> R) -> [`DoubleEndedIterator`] T -> R
#[macro_export]
macro_rules! foldr {
    ($init:expr,$f:expr) => {
        move |it| foldr($init,$f,it)
    };
    ($init:expr) => {
        move |f,it| foldr($init,f,it)
    };
    ($init:expr=>) => {
        move |f| (move |it| foldr($init,f,it))
    };
}

/// Curry macro of [filter](basic_fn::fun::filter)
/// 
/// **Signature**: filter :: (T -> bool) -> [`Iterator`] T -> [`Iterator`] T
#[macro_export]
macro_rules! filter {
    ($f:expr) => {
        move |it| filter($f, it)
    };
}

/// Curry macro of [filter_not](basic_fn::fun::filter_not)
/// 
/// **Signature**: filter_not :: (T -> bool) -> [`Iterator`]) T -> [`Iterator`] T
#[macro_export]
macro_rules! filter_not {
    ($f:expr) => {
        move |it| filter_not($f, it)
    };
}

/// This macro is used to provide shortcut of function composition.
/// The order is last-in-first-invoke.
/// 
/// # Arguments
/// 
/// * `args` - function or closure separated by comma(,)
#[macro_export]
macro_rules! compose {
    ($f:expr,$($fs:expr),*) => {
        compose!(@NEXT [$f],$($fs),*)
    };
    (@NEXT [$($fs:expr),*],$f:expr,$($remain_fs:expr),*) => {
        compose!(@NEXT [$f,$($fs),*],$($remain_fs),*)
    };
    (@NEXT [$($fs:expr),*],$f:expr) => {
        pipe!($f,$($fs),*)
    };
}

/// Shorthand of [skip](basic_fn::fun::skip)
///
/// **Signature**: skip :: [`usize`] -> [`Iterator`] T -> [`Iterator`] T
#[macro_export]
macro_rules! skip {
    ($n:expr) => {
        move |it| skip($n, it)
    };
}

/// Shorthand of [take](basic_fn::fun::take)
///
/// **Signature**: take :: [`usize`] -> [`Iterator`] T -> [`Iterator`] T
#[macro_export]
macro_rules! take {
    ($n:expr) => {
        move |it| take($n, it)
    };
}