//! # Copar - A COmmand PARser in rust
//! CoPar (Command Parser) allows to generate structured command lists in different languages from command logs complying with the CoPar language specification.  
//!
//! CoPar exists both as CLI and a library crate.
//! **Supported generated languages :** Rust, C#, C
//!
//! ## How to use it
//! Use the `Model::parse()` function to parse your logs.
//! Use any `Model::compute_to_*` function to generate the operation table in any available language.  
//! An example is available in `tests/file_gen`.
//!
//! ## Typical use case
//! You have a driver sending a complex series of command to a device for initializing it. Your goal is to reproduce the driver initialization. However you don't want to dive into understanding the complex drive code or maybe its source code is inaccessible.
//! You can insert CoPar-style operation records at key locations in the low-level access layer to log every operation going in and out of the driver. Give those line of logs to the parser to synthetize an array of operations in the language of your choice. It's up to you to write the program that will read and execute this array of operations.
//!
//!
//! ## CoPar language specification
//! ### Important definitions
//! **Record**: a text string in the sequence delimited by two tags (eg. `#< my_sequence <#`).
//! **Sequence delimiters**: special records used to delimit the start and end of a sequence.
//! **Sequence**: The text string that is parsed by the CoPar parser. Typically contains multiple operation records and free text.
//! **Operation**: Entry of the operation table having a name and a sequence of arguments.
//! **Operation table**: The list of operations generated in any available language from the complete sequence.
//! **Operation record**: record providing informations to generate an Operations
//!
//! ### Record types
//! Any character in the sequence outside the records will be ignored
//!
//! *Sequence delimiters:*
//! The CoPar parser will start scanning the text when encountering the "sequence begin" delimiter and will stop parsing when encountering the "sequence end" delimiter.
//!```grammar,lexer
//!     Sequence -> SequenceBegin
//!     SequenceBegin -> `#<` SEQUENCE_NAME `<#`
//!     SequenceEnd -> `#>` SEQUENCE_NAME `>#`
//! ```
//!
//! *One shot operation records*:
//! An operation represented by a single record:
//! `#= <OPERATION_NAME> (<ARG_NAME> <ARG_TYPE>(<ARG_VALUE>))* =#`
//!
//! Scope operation records:
//! An operation represented by a set of record.
//!   - Scope operation begins : `#< <OPERATION_NAME> <#`
//!   - Scope operation ends : `#> <OPERATION_NAME> >#`
//!   - Scope operation arguments: `#- <OPERATION_NAME> (<ARG_NAME> <ARG_VALUE>)* -#`

mod generators;
mod model;
mod parser;
mod unirecord;

pub use generators::c_generation::CGeneration;
pub use generators::c_sharp_generation::CSharpGeneration;
pub use generators::rust_generation::RustGeneration;
pub use model::Model;
pub use parser::Parser;
