#![license = "MIT"]
#![deny(missing_docs, warnings)]

//! Traits for unsafe downcasting from trait objects to & or &mut references of
//! concrete types. These should only be used if you are absolutely certain of the
//! type of the data in said trait object - there be dragons etc.
//!
//! Inspired by https://github.com/chris-morgan/anymap
//! and the implementation of `std::any::Any`.

use std::any::Any;
use std::mem;
use std::raw;

/// An extension trait for unchecked downcasting of trait objects to &T.
pub trait UncheckedAnyDowncast<'a> {
    /// Returns a reference to the boxed value, assuming that it is of type `T`. If you
    /// are not _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_ref_unchecked<T: 'static>(self) -> &'a T;
}

/// An extension trait for unchecked downcasting of trait objects to &mut T.
pub trait UncheckedAnyMutDowncast<'a> {
    /// Returns a mutable reference to the boxed value, assuming that it is of type `T`. If you
    /// are not _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_mut_unchecked<T: 'static>(self) -> &'a mut T;
}

/// An extension for unchecked downcasting of trait objects to Box<T>.
pub trait UncheckedBoxAnyDowncast {
    /// Return a box of type Box<T>, assuming the trait object contains a type T. If you are not
    /// _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_unchecked<T: 'static>(self) -> Box<T>;
}

impl<'a> UncheckedAnyDowncast<'a> for &'a Any {
    #[inline]
    unsafe fn downcast_ref_unchecked<T: 'static>(self) -> &'a T {
        // Cast to a trait object, get the data pointer, transmute to T.
        mem::transmute(mem::transmute::<&Any, raw::TraitObject>(self).data)
    }
}

impl<'a> UncheckedAnyMutDowncast<'a> for &'a mut Any {
    #[inline]
    unsafe fn downcast_mut_unchecked<T: 'static>(self) -> &'a mut T {
        // Cast to a trait object, get the data pointer, transmute to T.
        mem::transmute(mem::transmute::<&mut Any, raw::TraitObject>(self).data)
    }
}

impl UncheckedBoxAnyDowncast for Box<Any> {
    #[inline]
    unsafe fn downcast_unchecked<T: 'static>(self) -> Box<T> {
        let to = mem::transmute::<Box<Any>, raw::TraitObject>(self);

        mem::transmute(to.data)
    }
}

#[cfg(test)]
mod test {
    use super::{UncheckedAnyDowncast, UncheckedAnyMutDowncast, UncheckedBoxAnyDowncast};
    use std::any::Any;

    #[test] fn test_simple_downcast() {
        let a = box 7u as Box<Any>;
        unsafe { assert_eq!(*a.downcast_ref_unchecked::<uint>(), 7u); }
    }

    #[test] fn test_simply_mut_downcast() {
        let mut a = box 7u as Box<Any>;
        unsafe { assert_eq!(*a.downcast_mut_unchecked::<uint>(), 7u); }
    }

    #[test] fn test_mut_edit_through_downcast() {
        let mut a = box 7u as Box<Any>;
        unsafe {
            *a.downcast_mut_unchecked::<uint>() = 8u;
            assert_eq!(*a.downcast_mut_unchecked::<uint>(), 8u);
        }
    }

    #[test] fn test_box_downcast_no_double_free() {
        use std::sync::atomic::{AtomicUint, Ordering};
        use std::sync::Arc;

        struct Dropper {
            x: Arc<AtomicUint>
        }

        impl Drop for Dropper {
            fn drop(&mut self) {
                self.x.fetch_add(1, Ordering::SeqCst);
            }
        }

        let x = Arc::new(AtomicUint::new(0));
        let a = box Dropper { x: x.clone() } as Box<Any>;

        let dropper = unsafe { a.downcast_unchecked::<Dropper>() };
        drop(dropper);

        assert_eq!(x.load(Ordering::SeqCst), 1);
    }
}

