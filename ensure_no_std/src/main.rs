#![deny(warnings)]
#![allow(unused_macros)]

#![no_std]
#![no_main]

extern crate alloc;

#[cfg(windows)]
#[link(name="msvcrt")]
extern "C" { }

mod no_std {
    use composable_allocators::{AsGlobal, System};
    use core::panic::PanicInfo;
    use exit_no_std::exit;

    #[global_allocator]
    static ALLOCATOR: AsGlobal<System> = AsGlobal(System);

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        exit(99)
    }

    #[no_mangle]
    extern "C" fn rust_eh_personality() { }
}

use alloc::boxed::Box;
use dynamic_cast::{SupportsInterfaces, impl_supports_interfaces, dyn_cast_box};

// class Base

#[portrait::make]
trait IsBase: SupportsInterfaces {
    fn base_method(&self) -> u8;
}

struct Base {
    data: u8,
}

impl_supports_interfaces!(Base: IsBase);

impl IsBase for Base {
    fn base_method(&self) -> u8 { self.data }
}

// class Descendant: Base

#[portrait::make]
trait IsDescendant: IsBase + SupportsInterfaces {
    fn descendant_method(&self) -> u8;
}

struct Descendant {
    base: Base,
    data: u8,
}

impl_supports_interfaces!(Descendant: IsBase, IsDescendant);

#[portrait::fill(portrait::delegate(Base; self.base))]
impl IsBase for Descendant { }

impl IsDescendant for Descendant {
    fn descendant_method(&self) -> u8 { self.data }
}

// main

use core::ffi::{c_int, c_char};

#[no_mangle]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let a: Box<dyn IsDescendant> = Box::new(Descendant { base: Base { data: 1 }, data: 2 });
    assert_eq!(a.descendant_method(), 2);
    assert_eq!(a.base_method(), 1);
    let a_as_base: Box<dyn IsBase> = dyn_cast_box(a).unwrap();
    assert_eq!(a_as_base.base_method(), 1);
    let a_as_descendant: Box<dyn IsDescendant> = dyn_cast_box(a_as_base).unwrap();
    assert_eq!(a_as_descendant.descendant_method(), 2);
    assert_eq!(a_as_descendant.base_method(), 1);
    0
}
