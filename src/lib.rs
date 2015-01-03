#![deny(missing_docs, warnings)]

//! Traits for unsafe downcasting from trait objects to & or &mut references of
//! concrete types. These should only be used if you are absolutely certain of the
//! type of the data in said trait object - there be dragons etc.
//!
//! Originally inspired by https://github.com/chris-morgan/anymap
//! and the implementation of `std::any::Any`.

use std::any::Any;
use std::mem;
use std::raw;

/// A trait providing unchecked downcasting to its contents when stored
/// in a trait object.
pub trait UnsafeAny: Any {}
impl<T: Any> UnsafeAny for T {}

impl UnsafeAny {
    /// Returns a reference to the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    pub unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
        mem::transmute(mem::transmute::<&UnsafeAny, raw::TraitObject>(self).data)
    }

    /// Returns a mutable reference to the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    pub unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        mem::transmute(mem::transmute::<&mut UnsafeAny, raw::TraitObject>(self).data)
    }

    /// Returns a the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    pub unsafe fn downcast_unchecked<T: 'static>(self: Box<UnsafeAny>) -> Box<T> {
        mem::transmute(mem::transmute::<Box<UnsafeAny>, raw::TraitObject>(self).data)
    }
}

/// An extension trait for unchecked downcasting of trait objects.
pub trait UnsafeAnyExt for Sized? {
    /// Returns a reference to the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T;

    /// Returns a mutable reference to the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T;

    /// Returns a the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    unsafe fn downcast_unchecked<T: 'static>(self: Box<Self>) -> Box<T>;
}

impl UnsafeAnyExt for Any {
    unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
        mem::transmute(mem::transmute::<&Any, raw::TraitObject>(self).data)
    }

    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        mem::transmute(mem::transmute::<&mut Any, raw::TraitObject>(self).data)
    }

    unsafe fn downcast_unchecked<T: 'static>(self: Box<Any>) -> Box<T> {
        mem::transmute(mem::transmute::<Box<Any>, raw::TraitObject>(self).data)
    }
}

#[cfg(test)]
mod test {
    use super::{UnsafeAny, UnsafeAnyExt};
    use std::any::Any;

    #[test] fn test_simple_downcast_ext() {
        let a = box 7u as Box<Any>;
        unsafe { assert_eq!(*a.downcast_ref_unchecked::<uint>(), 7u); }

        let mut a = box 7u as Box<Any>;
        unsafe { assert_eq!(*a.downcast_mut_unchecked::<uint>(), 7u); }

        let mut a = box 7u as Box<Any>;
        unsafe {
            *a.downcast_mut_unchecked::<uint>() = 8u;
            assert_eq!(*a.downcast_mut_unchecked::<uint>(), 8u);
        }
    }

    #[test] fn test_simple_downcast_inherent() {
        let a = box 7u as Box<UnsafeAny>;
        unsafe { assert_eq!(*a.downcast_ref_unchecked::<uint>(), 7u); }

        let mut a = box 7u as Box<UnsafeAny>;
        unsafe { assert_eq!(*a.downcast_mut_unchecked::<uint>(), 7u); }

        let mut a = box 7u as Box<UnsafeAny>;
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
        let a = box Dropper { x: x.clone() } as Box<UnsafeAny>;

        let dropper = unsafe { a.downcast_unchecked::<Dropper>() };
        drop(dropper);

        assert_eq!(x.load(Ordering::SeqCst), 1);

        let x = Arc::new(AtomicUint::new(0));
        let a = box Dropper { x: x.clone() } as Box<Any>;

        let dropper = unsafe { a.downcast_unchecked::<Dropper>() };
        drop(dropper);

        assert_eq!(x.load(Ordering::SeqCst), 1);
    }
}

