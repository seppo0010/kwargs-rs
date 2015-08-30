# kwargs-rs

An experiment to have named optional arguments in Rust.

## Example

```rust
#[macro_use(def, call)] extern crate kwargs;

def!(test(v1: u8, v2: u8; operation:u8=1, reverse:bool=false) -> bool {
    reverse != match operation {
        1 => v1 == v2,
        2 => v1 != v2,
        3 => v1 > v2,
        4 => v1 < v2,
        _ => false,
    }
});

fn main() {
    assert!(call!(test(0, 0)));
    assert!(!call!(test(0, 1)));
    assert!(call!(test(0, 1; operation=2)));
    assert!(!call!(test(0, 1; operation=2, reverse=true)));
}
```
