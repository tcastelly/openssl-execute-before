mod execute_impl;
mod execute_io;

pub use execute_impl::*;

pub mod prelude {
    pub use crate::openssl::execute_io::*;
}
