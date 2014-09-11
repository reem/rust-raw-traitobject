#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! Unsafe helpers for working with raw TraitObjects.

use std::raw::TraitObject;
use std::mem;
use std::any::Any;

/// Get a trait object.
pub fn abstract<T>(val: &T) -> &Any { val as &Any }

/// Get the size of the underlying type.
pub fn size(val: &Any) -> uint {
    let (_, size, _) = unsafe { *metadata(val) };
    size
}

/// Get the alignment of the underlying type.
pub fn alignment(val: &Any) -> uint {
    let (_, _, alignment) = unsafe { *metadata(val) };
    alignment
}

/// Get the destructor of this type.
///
/// It is highly unsafe to call the destructor.
pub fn destructor(val: &Any) -> unsafe fn(*mut ()) {
    let (dtor, _, _) = unsafe { *metadata(val) };
    dtor
}

/// Get the data pointer from this trait object.
///
/// This is highly unsafe as there is no information
/// about the type of this data.
pub unsafe fn data(val: &Any) -> &mut () {
    mem::transmute(raw(val).data)
}

/// Get the vtable from this trait object.
///
/// This is highly unsafe as there is no information
/// about the size of the vtable.
pub unsafe fn vtable(val: &Any) -> &mut () {
    mem::transmute(raw(val).vtable)
}

/// Get this reference as a raw::TraitObject
pub unsafe fn raw(val: &Any) -> TraitObject {
    mem::transmute_copy(&val)
}

/// Get the metadata from this trait objects vtable.
///
/// This is represented as (destructor, size, alignment)
pub unsafe fn metadata(val: &Any) -> &mut (fn(*mut ()), uint, uint) {
    mem::transmute(vtable(val))
}

#[cfg(test)]
mod test {
    use {abstract, size, alignment};
    struct Sized { _x: uint }

    #[test]
    fn test_size() {
        assert_eq!(size(abstract(&Sized { _x: 7u })), 8u)
    }

    #[test]
    fn test_alignment() {
        assert_eq!(alignment(abstract(&Sized { _x: 7u })), 8u)
    }
}

