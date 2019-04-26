Currently a very simple crate to generate error impls from unit structs.

# Examples

```rust
use error_gen::ErrorGen;

#[derive(Debug, ErrorGen)]
pub struct MyError;

fn main() {
    let e = MyError;
    assert_eq!(e.to_string(), "my error");
    assert!(std::error::Error::source(e).is_none());
}
```
