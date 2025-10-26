//! # Copar - A COmmand PARser in rust
//! CoPar (Command Parser) allows to generate structured command lists in
//! different languages from command logs complying with the CoPar language specification.
mod generators;
mod model;
mod parser;
mod unirecord;

pub use generators::c_generation::CGeneration;
pub use generators::c_sharp_generation::CSharpGeneration;
pub use generators::rust_generation::RustGeneration;
pub use model::Model;
pub use parser::Parser;
