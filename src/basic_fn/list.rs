// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// This macro is used to provide ability of list comprehension.
/// Return Vec<T>.
/// 
/// Format:
/// ls![{Mapper};{Iter}=>{Filterer}]
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
/// * `Iter` - Iterator<T>
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