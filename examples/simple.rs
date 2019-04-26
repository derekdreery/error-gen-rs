use error_gen::ErrorGen;
use std::error::Error;

#[derive(Debug, ErrorGen)]
pub struct ExampleError;

fn main() {
    let e = ExampleError;
    assert_eq!(e.to_string(), "example error");
    assert!(e.source().is_none());
}
