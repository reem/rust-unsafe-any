#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! Traits for unsafe downcasting from trait objects to & or &mut references of
//! concrete types. These should only be used if you are absolutely certain of the
//! type of the data in said trait object - there be dragons etc.
//!
//! *Heavily* inspired by https://github.com/chris-morgan/anymap
//! but refactored and re-exported for modularity.

use std::any::Any;
use std::mem;
use std::raw;

/// An extension trait for unchecked downcasting of trait objects to &T.
pub trait UncheckedAnyDowncast<'a> {
    /// Returns a reference to the boxed value, assuming that it is of type `T`. If you
    /// are not _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_ref_unchecked<T: Any>(self) -> &'a T;
}

/// An extension trait for unchecked downcasting of trait objects to &mut T.
pub trait UncheckedAnyMutDowncast<'a> {
    /// Returns a mutable reference to the boxed value, assuming that it is of type `T`. If you
    /// are not _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_mut_unchecked<T: Any>(self) -> &'a mut T;
}

impl<'a> UncheckedAnyDowncast<'a> for &'a Any {
    #[inline]
    unsafe fn downcast_ref_unchecked<T: Any>(self) -> &'a T {
        // Cast to a trait object, get the data pointer, transmute to T.
        mem::transmute(mem::transmute_copy::<&Any, raw::TraitObject>(&self).data)
    }
}

impl<'a> UncheckedAnyMutDowncast<'a> for &'a mut Any {
    #[inline]
    unsafe fn downcast_mut_unchecked<T: Any>(self) -> &'a mut T {
        // Cast to a trait object, get the data pointer, transmute to T.
        mem::transmute(mem::transmute_copy::<&mut Any, raw::TraitObject>(&self).data)
    }
}

