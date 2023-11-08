#![feature(allocator_api)]
#![feature(ptr_metadata)]

#![no_std]

#[doc=include_str!("../README.md")]
type _DocTestReadme = ();

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use arraybox::{ArrayBox, BufFor};
use downcast_rs::{Downcast, impl_downcast};
use core::alloc::Allocator;
use core::any::TypeId;
use core::ops::Deref;
use core::ptr::{self, DynMetadata, Pointee};

#[doc(hidden)]
pub use core::any::TypeId as core_any_TypeId;
#[doc(hidden)]
pub use core::option::Option as core_option_Option;

pub type BoxedInterfaceMetadata =
    ArrayBox<'static, dyn IsInterfaceMetadata, BufFor<InterfaceMetadata<dyn IsInterfaceMetadata>>>
;

pub trait IsInterfaceMetadata: Downcast { }

impl_downcast!(IsInterfaceMetadata);

pub struct InterfaceMetadata<DynInterface: ?Sized>(pub DynMetadata<DynInterface>);

impl<DynInterface: ?Sized + 'static> IsInterfaceMetadata for InterfaceMetadata<DynInterface> { }

pub unsafe trait SupportsInterfaces {
    fn get_interface_metadata(&self, dyn_interface_id: TypeId) -> Option<BoxedInterfaceMetadata>;
}

pub fn dyn_cast_box<T: SupportsInterfaces + ?Sized, DynInterface: ?Sized + 'static, A: Allocator>(
    x: Box<T, A>
) -> Option<Box<DynInterface, A>> where DynInterface: Pointee<Metadata=DynMetadata<DynInterface>> {
    unsafe { dyn_cast_raw_mut(x, Box::into_raw_with_allocator, Box::from_raw_in) }
}

pub fn dyn_cast_rc<T: SupportsInterfaces + ?Sized, DynInterface: ?Sized + 'static>(
    x: Rc<T>
) -> Option<Rc<DynInterface>> where
    DynInterface: Pointee<Metadata=DynMetadata<DynInterface>>,
    T: Pointee<Metadata=DynMetadata<T>>
{
    unsafe { dyn_cast_raw(x, |x| (Rc::into_raw(x), ()), |x, ()| Rc::from_raw(x)) }
}

pub fn dyn_cast_ref<T: SupportsInterfaces + ?Sized, DynInterface: ?Sized + 'static>(
    x: &T
) -> Option<&DynInterface> where DynInterface: Pointee<Metadata=DynMetadata<DynInterface>> {
    unsafe { dyn_cast_raw(x, |x| (x as *const T, ()), |x, ()| &*x) }
}

pub fn dyn_cast_mut<T: SupportsInterfaces + ?Sized, DynInterface: ?Sized + 'static>(
    x: &mut T
) -> Option<&mut DynInterface> where DynInterface: Pointee<Metadata=DynMetadata<DynInterface>> {
    unsafe { dyn_cast_raw_mut(x, |x| (x as *mut T, ()), |x, ()| &mut *x) }
}

pub unsafe fn dyn_cast_raw_mut<
    T: SupportsInterfaces + ?Sized,
    DynInterface: ?Sized + 'static,
    X: Deref<Target=T>,
    Y,
    A
>(
    x: X,
    into_raw_parts: fn(X) -> (*mut T, A),
    from_raw_parts: unsafe fn(*mut DynInterface, A) -> Y
) -> Option<Y> where DynInterface: Pointee<Metadata=DynMetadata<DynInterface>> {
    let metadata = x.get_interface_metadata(TypeId::of::<DynInterface>())?;
    let metadata = metadata.downcast_ref::<InterfaceMetadata<DynInterface>>()
        .unwrap_or_else(|| panic!("invalid get_dyn_cast_metadata implementation"))
        .0
    ;
    let (raw_ptr, a) = into_raw_parts(x);
    let raw_ptr = raw_ptr.to_raw_parts().0;
    let raw_ptr = ptr::from_raw_parts_mut(raw_ptr, metadata);
    let x = from_raw_parts(raw_ptr, a);
    Some(x)
}

pub unsafe fn dyn_cast_raw<
    T: SupportsInterfaces + ?Sized,
    DynInterface: ?Sized + 'static,
    X: Deref<Target=T>,
    Y,
    A
>(
    x: X,
    into_raw_parts: fn(X) -> (*const T, A),
    from_raw_parts: unsafe fn(*const DynInterface, A) -> Y
) -> Option<Y> where DynInterface: Pointee<Metadata=DynMetadata<DynInterface>> {
    let metadata = x.get_interface_metadata(TypeId::of::<DynInterface>())?;
    let metadata = metadata.downcast_ref::<InterfaceMetadata<DynInterface>>()
        .unwrap_or_else(|| panic!("invalid get_dyn_cast_metadata implementation"))
        .0
    ;
    let (raw_ptr, a) = into_raw_parts(x);
    let raw_ptr = raw_ptr.to_raw_parts().0;
    let raw_ptr = ptr::from_raw_parts(raw_ptr, metadata);
    let x = from_raw_parts(raw_ptr, a);
    Some(x)
}

#[inline]
pub fn try_get_interface_metadata_for<DynInterface: ?Sized + 'static>(
    dyn_interface_id: TypeId,
    this: &DynInterface,
) -> Option<BoxedInterfaceMetadata> where DynInterface: Pointee<Metadata=DynMetadata<DynInterface>> {
    if dyn_interface_id == TypeId::of::<DynInterface>() {
        Some(ArrayBox::new(InterfaceMetadata(ptr::metadata(this as *const DynInterface))))
    } else {
        None
    }
}

#[macro_export]
macro_rules! impl_supports_interfaces {
    (
        $name:ty $(: $($($interface:path),+ $(,)?)?)?
    ) => {
        unsafe impl $crate::SupportsInterfaces for $name {
            fn get_interface_metadata(
                &self,
                dyn_interface_id: $crate::core_any_TypeId
            ) -> $crate::core_option_Option<$crate::BoxedInterfaceMetadata> {
                $($($(
                    if
                        let $crate::core_option_Option::Some(metadata) =
                            $crate::try_get_interface_metadata_for::<dyn $interface>(
                                dyn_interface_id, self
                            )
                    {
                        return $crate::core_option_Option::Some(metadata);
                    }
                )+)?)?
                None
            }
        }
    };
}
