#![deny(missing_docs)]
//! Utility for recursively unzipping [`tuple`]s, [`Option`]s of [`tuple`]s and
//! [`Result`]s of [`tuple`]s.
//!
//! # Usage
//! This crate is quiet straightforward.
//!
//! ## Unzipping `(((A, B), C), ...)`
//! If you have a left- or right-recursively zipped tuple, you can use
//! [`UnzipInto::unzip_into`] to turn it into a non-recursive tuple. This works
//! for up to 26 tuple elements.
//!
//! ```rust
//! use zipped::UnzipInto;
//!
//! let (a, b, c) = ((1, 2), 3).unzip_into(); // left-recursive
//! let (a, b, c) = (1, (2, 3)).unzip_into(); // right-recursive
//! ```
//!
//! ### Unzipping `Option<(((A, B), C), ...)>`
//! If you have an [`Option`] that contains a left- or right-recursively zipped
//! tuple, you can also use [`UnzipInto::unzip_into`] to turn it into an
//! [`Option`] of a non-recursive tuple. This also works for up to 26 tuple
//! elements.
//!
//! ```rust
//! use zipped::UnzipInto;
//!
//! let zipped = Some(1).zip(Some(2)).zip(Some(3));
//!
//! match zipped.unzip_into() {
//!     Some((a, b, c)) => {}
//!     None => {}
//! }
//! ```
//!
//! While it's also possible to unzip [`Option`]s with right-recursively zipped
//! tuples, these don't occur naturally since [`Option::zip`] is left-recursive.
//!
//! ### Unzipping `Result<(((A, B), C), ...), E>`
//! If you have a [`Result`] that contains a left- or right-recursively zipped
//! tuple, you can also use [`UnzipInto::unzip_into`] to turn it into a
//! [`Result`] of a non-recursive tuple. This also works for up to 26 tuple
//! elements.
//!
//! ```rust
//! use zipped::UnzipInto;
//!
//! let zipped = Ok::<_, ()>(1)
//!     .and_then(|a| Ok((a, 2)))
//!     .and_then(|ab| Ok((ab, 3)));
//!
//! match zipped.unzip_into() {
//!     Ok((a, b, c)) => {}
//!     Err(_) => {}
//! }
//! ```
//!
//! Again, while it's also possible to unzip [`Result`]s with right-recursively
//! zipped tuples, I found that these occur much less often.
//!
//! # Limitations
//! - __Type inference.__ The compiler cannot automatically infer `T` in
//!   [`UnzipInto<T>`]. Eventually, you will need to specify the return value's
//!   arity.
//! - __Maximum arity.__ [`UnzipFrom`] is implemented for tuples of up to 26
//!   elements.
//! - __Strict.__ It only works for completely zipped tuples where each tuple
//!   contains 2 elements and only the left (or the right) element can be
//!   another tuple, i.e. it does not work for `((A, B), C, D)`.

/// Value-to-value conversion analogous to [`Into`] that consumes an input value
/// of this type and returns an unzipped equivalent of type `T`.
pub trait UnzipInto<T> {
    /// Unzips this type into `T`.
    fn unzip_into(self) -> T;
}

impl<T, U> UnzipInto<T> for U
where
    T: UnzipFrom<U>,
{
    fn unzip_into(self) -> T {
        T::unzip_from(self)
    }
}

/// Value-to-value conversation analogous to [`From`] that consumes an input
/// value of type `T` and returns an unzipped equivalent of this type.
pub trait UnzipFrom<T>
where
    T: ?Sized,
{
    /// Unzips `T` into this type.
    fn unzip_from(tuple: T) -> Self;
}

macro_rules! left {
    ($a:tt) => { $a };
    ($a:tt $b:ident) => { ($a, $b) };
    ($a:tt $b:ident $($c:ident)*) => { left!(($a, $b) $($c)*) };
}

macro_rules! right {
    ($a:tt) => { $a };
    ($a:tt $b:ident) => { ($a, $b) };
    ($a:tt $b:ident $($c:ident)*) => { ($a, ($b, right!($($c)*))) };
}

macro_rules! nested {
    ($a:ident $b:ident $c:ident $($ident:ident)*) => {
        nested!(@ $a $b $c ; $($ident)*);
    };
    (@ $($ident:ident)* ; $next:ident $($rest:ident)*) => {
        nested!(@ $($ident)* ;);
        nested!(@ $($ident)* $next ; $($rest)*);
    };
    (@ $($ident:ident)* ;) => {
        #[allow(non_snake_case)]
        impl<$($ident,)*> UnzipFrom<left!($($ident)*)> for ($($ident,)*) {
            fn unzip_from(zip: left!($($ident)*)) -> Self {
                let left!($($ident)*) = zip;
                ($($ident,)*)
            }
        }

        #[allow(non_snake_case)]
        impl<$($ident,)*> UnzipFrom<right!($($ident)*)> for ($($ident,)*) {
            fn unzip_from(zip: right!($($ident)*)) -> Self {
                let right!($($ident)*) = zip;
                ($($ident,)*)
            }
        }
    }
}

impl<T, U> UnzipFrom<Option<T>> for Option<U>
where
    U: UnzipFrom<T>,
{
    fn unzip_from(tuple: Option<T>) -> Self {
        tuple.map(UnzipFrom::unzip_from)
    }
}

impl<T, E, U> UnzipFrom<Result<T, E>> for Result<U, E>
where
    U: UnzipFrom<T>,
{
    fn unzip_from(tuple: Result<T, E>) -> Self {
        tuple.map(UnzipFrom::unzip_from)
    }
}

impl<A> UnzipFrom<(A,)> for (A,) {
    fn unzip_from(tuple: (A,)) -> Self {
        tuple
    }
}

impl<A, B> UnzipFrom<(A, B)> for (A, B) {
    fn unzip_from(tuple: (A, B)) -> Self {
        tuple
    }
}

nested!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::{UnzipFrom, UnzipInto};

    #[test]
    fn test_left_recursive_tuple() {
        let (x, y, z) = ((1, 2), 3).unzip_into();
    }

    #[test]
    fn test_right_recursive_tuple() {
        let (x, y, z) = (1, (2, 3)).unzip_into();
    }

    #[test]
    fn test_left_recursive_option() {
        let zipped = Some(false).zip(Some(false)).zip(Some(false));

        match zipped.unzip_into() {
            Some((a, b, c)) => {}
            None => {}
        }
    }

    #[test]
    fn test_left_recursive_result() {
        let zipped = Ok::<_, ()>(false)
            .and_then(|a| Ok((a, false)))
            .and_then(|ab| Ok((ab, false)));

        match zipped.unzip_into() {
            Ok((a, b, c)) => {}
            Err(_) => {}
        }
    }
}
