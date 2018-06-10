// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Curry macro of [map](basic_fn::fun::map)
/// 
/// **Signature**: map :: (T -> U) -> [`Iterator`] T -> [`Iterator`] U
#[macro_export] macro_rules! map {
    ($f:expr) => {
        move |it| map($f, it)
    };
}

/// Shorthand macro of sum
/// 
/// Syntax:
/// 1. sum!(0;5) // equals sum(0..=5)
/// 2. sum!(0,1,2,3,4,5)
#[macro_export] macro_rules! sum {
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
#[macro_export] macro_rules! pipe  {
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
#[macro_export] macro_rules! ls {
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
#[macro_export] macro_rules! foldl {
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
#[macro_export] macro_rules! foldr {
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
#[macro_export] macro_rules! filter {
    ($f:expr) => {
        move |it| filter($f, it)
    };
}

/// Curry macro of [filter_not](basic_fn::fun::filter_not)
/// 
/// **Signature**: filter_not :: (T -> bool) -> [`Iterator`]) T -> [`Iterator`] T
#[macro_export] macro_rules! filter_not {
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
#[macro_export] macro_rules! compose {
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
#[macro_export] macro_rules! skip {
    ($n:expr) => {
        move |it| skip($n, it)
    };
}

/// Shorthand of [take](basic_fn::fun::take)
///
/// **Signature**: take :: [`usize`] -> [`Iterator`] T -> [`Iterator`] T
#[macro_export] macro_rules! take {
    ($n:expr) => {
        move |it| take($n, it)
    };
}

/// Shorthand macro of [product](basic_fn::fun::product)
///
/// Syntax:
/// 1. product!(0;5) // equals product(0..=5)
/// 2. product!(0,1,2,3,4,5)
#[macro_export] macro_rules! product {
    ($i:expr;$j:expr) => {{
        product($i..=$j)
    }};
    ($i:expr,$($j:expr),*) => {{
        $i * product!($($j),*)
    }};
    ($i:expr) => {
        $i
    };
}

/// Extend [concat](basic_fn::fun::concat)
///
/// Syntax:
/// concat!(it1;it2;it3...)
#[macro_export] macro_rules! concat {
    ($($it:expr);*) => {{
        let mut ret = Vec::new();
        $(ret.extend($it););*
        ret
    }};
}

/// Get the first element of [`tuple`]
///
/// Syntax:
/// 1. fst!(>type,type...) :: (a,b,...) -> a
/// 2. fst!((a,b,...)|) -> a
/// 3. let x = (a,b); fst!(x) -> a
#[macro_export] macro_rules! fst {
    (>$($t:ty),*) => {move |x: ($($t),*)| x.0};
    ($x:expr) => {{$x.0}};
    ($x:pat) => {{$x.0}};
}

/// Get the second element of [`tuple`]
///
/// Syntax:
/// 1. snd!(>type,type...) :: (a,b,...) -> b
/// 2. snd!((a,b,...)|) -> b
/// 3. let x = (a,b); snd!(x) -> b
#[macro_export] macro_rules! snd {
    (>$($t:ty),*) => {move |x: ($($t),*)| x.1};
    ($x:expr) => {{$x.1}};
    ($x:pat) => {{$x.1}};
}

/// Get reminder of division
#[macro_export] macro_rules! rem {
    ($x:expr,$y:expr) => {{rem($x, $y)}};
    ($x:expr) => {move |y| rem($x, y)};
}

/// Absolute of signed
///
/// Syntax:
/// 1. abs(>type) :: Signed type => type -> type
/// 2. abs(x) -> x
#[macro_export] macro_rules! abs {
    (>$t:ty) => {move |x: $t| x.abs()};
    ($x:expr) => {{$x.abs()}};
}

/// Signum of signed
///
/// Syntax:
/// 1. signum(>type) :: Signed type => type -> -1|0|1
/// 2. signum(x) -> -1|0|1
#[macro_export] macro_rules! signum {
    (>$t:ty) => {move |x: $t| x.signum()};
    ($x:expr) => {{$x.signum()}};
}

/// check if a number is even
///
/// Syntax:
/// 1. even!(x) -> bool
/// 2. even!(>type) :: Integer -> bool
#[macro_export] macro_rules! even {
    (>$t:ty) => {move |x: $t| x % 2 == 0};
    ($x:expr) => {{$x % 2 == 0}};
}

/// check if a number is odd
///
/// Syntax:
/// 1. odd!(x) -> bool
/// 2. odd!(>type) :: Integer -> bool
#[macro_export] macro_rules! odd {
    (>$t:ty) => {move |x: $t| x % 2 == 1};
    ($x:expr) => {{$x % 2 == 1}};
}

/// 1/x
///
/// Syntax:
/// 1. recip!(>type) :: T -> T
/// 2. recip!(x) = 1/x
///
/// # Panics
/// if x == 0
#[macro_export] macro_rules! recip {
    (>$t:ty) => {move |x: $t| 1/$t};
    ($x:expr) => {{1/$x}};
}

/// always return first memorized values
///
/// Syntax:
/// 1. always!() :: a -> Any -> a
/// 2. always!(=>) :: a -> (Any -> a)
/// 3. always!(x) :: Any -> x
/// 4. always!(x,_) = x
#[macro_export] macro_rules! always {
    ($x:expr,$y:expr) => {{$x}};
    ($x:expr) => {move |_| $x};
    () => {move |x,_| x};
    (=>) => {move |x| (move |_| x)};
}

/// x + y
///
/// Syntax:
/// 1. add!(x) :: [`Add`] t => t -> t
/// 2. add!(x,y) = x + y
#[macro_export] macro_rules! add {
    ($x:expr) => {move |y| $x + y};
    ($x:expr,$y:expr) => {{$x + $y}};
}

/// x - y
///
/// Syntax:
/// 1. sub!(x) :: [`Sub`] t => t -> t
/// 2. sub!(x,y) = x - y
#[macro_export] macro_rules! sub {
    ($x:expr) => {move |y| $x - y};
    ($x:expr,$y:expr) => {{$x - $y}};
}

/// x * y
///
/// Syntax:
/// 1. mul!(x) :: [`Mul`] t => t -> t
/// 2. mul!(x,y) = x * y
#[macro_export] macro_rules! mul {
    ($x:expr) => {move |y| $x * y};
    ($x:expr,$y:expr) => {{$x * $y}};
}

/// x / y
///
/// Syntax:
/// 1. div!(x) :: [`Div`] t => t -> t
/// 2. div!(x,y) = x / y
#[macro_export] macro_rules! div {
    ($x:expr) => {move |y| $x / y};
    ($x:expr,$y:expr) => {{$x / $y}};
}

/// macro of [find](basic_fn::fun::find)(k, iter)
///
/// Syntax:
/// 1. find!(k) :: [`Iterator`] (K,V) -> Option (K,V)
/// 2. find!(k,iter) = find(k,iter)
#[macro_export] macro_rules! find {
    ($k:expr) => {move |it| find($k,it)};
    ($k:expr,$it:expr) => {{find($k,$it)}};
}

/// macro of [sorted_by](basic_fn::fun::sorted_by)
/// 
/// Syntax:
/// sorted_by!(f) :: [`Iterator`] T -> [`Iterator`] T
#[macro_export] macro_rules! sorted_by {
    ($f:expr) => {move |it| sorted_by($f, it)};
}

/// macro of sorted_with
/// 
/// fs = [f], f :: (&T, &T) -> Ordering
/// 
/// Syntax:
/// 1. sorted_with!(fs,...) :: [`Iterator`] T -> [`Iterator`] T
/// 2. sorted_with!(fs,...;it) = [`Iterator`] T
// TODO - to be continued
// #[macro_export] macro_rules! sorted_with {
//     ($($f:expr),*) => {move |it| sorted_with!($($f),*;it)};
//     ($head:expr,$($f:expr),*;$it:expr) => {{
//         use std::cmp::Ordering;
//         for move |x,y| {
//             let tmp = $head(x,y);
//             if tmp == Ordering::Equal {
//                 sorted_with!($($f),*;$it)
//             } else {
//                 tmp
//             }
//         }
//     }};
// }

/// macro of [zip](basic_fn::fun::zip)
/// 
/// Syntax:
/// zip!(it) :: [`Iterator`] U -> [`Iterator`] (T,U)
#[macro_export] macro_rules! zip {
    ($it:expr) => {move |it| zip($it, it)};
}

/// macro of [zip_with](basic_fn::fun::zip_with)
/// 
/// Syntax:
/// 1. zip_with!(f) :: ([`Iterator`] T -> [`Iterator`] U -> [`Iterator`] V) -> [`Iterator`] V
/// 2. zip_with!(f=>) :: [`Iterator`] T -> [`Iterator`] U -> [`Iterator`] V
/// 3. zip_with!(f, it) :: [`Iterator`] U -> [`Iterator`] V
#[macro_export] macro_rules! zip_with {
    ($f:expr) => {move |it1, it2| zip_with($f, it1, it2)};
    ($f:expr=>) => {move |it1| (move |it2| zip_with($f, it1, it2))};
    ($f:expr,$it:expr) => {move |it| zip_with($f, $it, it)};
}