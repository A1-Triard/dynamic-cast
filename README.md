![maintenance: experimental](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# dynamic-cast

The fifth pillar of OOP: dynamic casting.

```rust
use dynamic_cast::{SupportsInterfaces, impl_supports_interfaces, dyn_cast_box};

// class Base

#[portrait::make]
trait TBase: SupportsInterfaces {
    fn base_method(&self) -> u8;
}

struct Base {
    data: u8,
}

impl_supports_interfaces!(Base: TBase);

impl TBase for Base {
    fn base_method(&self) -> u8 { self.data }
}

// class Descendant: Base

#[portrait::make]
trait TDescendant: TBase + SupportsInterfaces {
    fn descendant_method(&self) -> u8;
}

struct Descendant {
    base: Base,
    data: u8,
}

impl_supports_interfaces!(Descendant: TBase, TDescendant);

#[portrait::fill(portrait::delegate(Base; self.base))]
impl TBase for Descendant { }

impl TDescendant for Descendant {
    fn descendant_method(&self) -> u8 { self.data }
}

// casting

fn main() {
    let a: Box<dyn TDescendant> = Box::new(Descendant { base: Base { data: 1 }, data: 2 });
    assert_eq!(a.descendant_method(), 2);
    assert_eq!(a.base_method(), 1);
    let a_as_base: Box<dyn TBase> = dyn_cast_box(a).unwrap();
    assert_eq!(a_as_base.base_method(), 1);
    let a_as_descendant: Box<dyn TDescendant> = dyn_cast_box(a_as_base).unwrap();
    assert_eq!(a_as_descendant.descendant_method(), 2);
    assert_eq!(a_as_descendant.base_method(), 1);
}
```
