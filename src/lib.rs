#![deny(missing_docs, warnings)]

//! Traits for unsafe downcasting from trait objects to & or &mut references of
//! concrete types. These should only be used if you are absolutely certain of the
//! type of the data in said trait object - there be dragons etc.
//!
//! Originally inspired by https://github.com/chris-morgan/anymap
//! and the implementation of `std::any::Any`.

extern crate traitobject;

use std::any::Any;
use std::mem;

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
        mem::transmute(traitobject::data(self))
    }

    /// Returns a mutable reference to the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    pub unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        mem::transmute(traitobject::data_mut(self))
    }

    /// Returns a the contained value, assuming that it is of type `T`.
    ///
    /// ## Warning
    ///
    /// If you are not _absolutely certain_ of `T` you should _not_ call this!
    pub unsafe fn downcast_unchecked<T: 'static>(self: Box<UnsafeAny>) -> Box<T> {
        let raw: *mut UnsafeAny = mem::transmute(self);
        mem::transmute(traitobject::data_mut(raw))
    }
}

/// An extension trait for unchecked downcasting of trait objects.
pub trait UnsafeAnyExt {
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
        mem::transmute(traitobject::data(self))
    }

    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        mem::transmute(traitobject::data_mut(self))
    }

    unsafe fn downcast_unchecked<T: 'static>(self: Box<Any>) -> Box<T> {
        let raw: *mut UnsafeAny = mem::transmute(self);
        mem::transmute(traitobject::data_mut(raw))
    }
}

#[cfg(test)]
mod test {
    use super::{UnsafeAny, UnsafeAnyExt};
    use std::any::Any;

    #[test] fn test_simple_downcast_ext() {
        let a = Box::new(7) as Box<Any>;
        unsafe { assert_eq!(*a.downcast_ref_unchecked::<usize>(), 7); }

        let mut a = Box::new(7) as Box<Any>;
        unsafe { assert_eq!(*a.downcast_mut_unchecked::<usize>(), 7); }

        let mut a = Box::new(7) as Box<Any>;
        unsafe {
            *a.downcast_mut_unchecked::<usize>() = 8;
            assert_eq!(*a.downcast_mut_unchecked::<usize>(), 8);
        }
    }

    #[test] fn test_simple_downcast_inherent() {
        let a = Box::new(7) as Box<UnsafeAny>;
        unsafe { assert_eq!(*a.downcast_ref_unchecked::<usize>(), 7); }

        let mut a = Box::new(7) as Box<UnsafeAny>;
        unsafe { assert_eq!(*a.downcast_mut_unchecked::<usize>(), 7); }

        let mut a = Box::new(7) as Box<UnsafeAny>;
        unsafe {
            *a.downcast_mut_unchecked::<usize>() = 8;
            assert_eq!(*a.downcast_mut_unchecked::<usize>(), 8);
        }
    }

    #[test] fn test_box_downcast_no_double_free() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        struct Dropper {
            x: Arc<AtomicUsize>
        }

        impl Drop for Dropper {
            fn drop(&mut self) {
                self.x.fetch_add(1, Ordering::SeqCst);
            }
        }

        let x = Arc::new(AtomicUsize::new(0));
        let a = Box::new(Dropper { x: x.clone() }) as Box<UnsafeAny>;

        let dropper = unsafe { a.downcast_unchecked::<Dropper>() };
        drop(dropper);

        assert_eq!(x.load(Ordering::SeqCst), 1);

        let x = Arc::new(AtomicUsize::new(0));
        let a = Box::new(Dropper { x: x.clone() }) as Box<Any>;

        let dropper = unsafe { a.downcast_unchecked::<Dropper>() };
        drop(dropper);

        assert_eq!(x.load(Ordering::SeqCst), 1);
    }
}

