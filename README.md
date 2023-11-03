![maintenance: experimental](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# dynamic-cast

```rust
#[portrait::make]
trait TBase: SupportsInterfaces {
    fn base_method(&self) -> u8;
}

struct Base {
    data: u8,
}

impl TBase for Base {
    fn base_method(&self) -> u8 { self.data }
}

object!(Base: TBase);

#[portrait::make]
trait TDescendant: TBase + Object {
    fn descendant_method(&self) -> u8;
}

struct Descendant {
    base: Base,
    data: u8,
}

#[portrait::fill(portrait::delegate(Base; self.base))]
impl TBase for Descendant { }

impl TDescendant for Descendant {
    fn descendant_method(&self) -> u8 { self.data }
}

object!(Descendant: TBase, TDescendant);

fn main() {
    let a: Box<dyn TBase> = Box::new(Descendant { base: Base { data: 1 }, data: 2 });
    let a_as_descendant: Box<dyn TDescendant> = dyn_cast(a).unwrap();
    assert_eq!(a_as_descendant.descendant_method(), 2);
    assert_eq!(a_as_descendant.base_method(), 1);
}
```
