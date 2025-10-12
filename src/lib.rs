mod code_generation_commons;
mod generators;
mod model;
mod parser;
mod unirecord;

pub use generators::c_generation::CGeneration;
pub use generators::c_sharp_generation::CSharpGeneration;
pub use generators::rust_generation::RustGeneration;
pub use model::Model;
