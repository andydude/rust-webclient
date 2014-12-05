#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[path = "."]
pub mod webclient {
    pub mod bits;
    pub mod digest;
}
