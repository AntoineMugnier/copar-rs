# Copar - A COmmand Parser in rust
CoPar (Command Parser) allows to generate structured command lists in different languages from command logs complying with the CoPar language specification.  

CoPar exists both as CLI and a library crate.
**Supported generated languages :** Rust, C#, C

## How to use the library
Use the `Model::parse()` function to parse your logs. 
Use any `Model::compute_to_*` function to generate the command table in any available language.  
An example is available in `tests/file_gen`.

## Typical use case
Imagine you have a driver sending a complex series of command to a device for initializing it. Your goal is to reproduce the device initialization. However you don't want to dive into understanding the complex driver code or maybe its closed source.
You can insert CoPar-style command records at key locations in the low-level access layer to log every command going out of the driver. Give those line of logs to the parser to synthesize an array of commands in the language of your choice. It's up to you to write the program that will read and execute this array of commands.

## CoPar language specification
### Important definitions
**Record**: a text string in the sequence delimited by two tags (eg. `#< my_sequence <#`).

**Sequence delimiters**: special records used to delimit the start and end of a sequence.

**Sequence**: The text string that is parsed by the CoPar parser. Typically contains multiple command records and free text.

**Command**: Entry of the command table having a name and a sequence of arguments.

**Command table**: The list of commands generated in any available language from the complete sequence.

**Command record**: record providing informations to generate an Commands

### Record types
Any character in the sequence outside the records will be ignored

**Sequence delimiters:**
The CoPar parser will start scanning the text when encountering the "sequence begin" delimiter and will stop parsing when encountering the "sequence end" delimiter. 

``` 
SEQUENCE_BEGIN → `#< SEQUENCE_NAME <#`
SEQUENCE_END → `#> SEQUENCE_NAME >#`
SEQUENCE_NAME → IDENTIFIER
``` 

**One shot command records**:
A command represented by a single record:

``` 
ONE_SHOT_CMD_RECORD → #= COMMAND_NAME (ARG_NAME ARG)* =#
``` 

**Scope command records:**
A command represented by a set of record.
``` 
SCOPE_CMD_RECORD_BEGIN → #< COMMAND_NAME <#`
SCOPE_CMD_RECORD →  #- COMMAND_NAME (ARG_NAME ARG)* -#
SCOPE_CMD_RECORD_END → #> COMMAND_NAME >#
``` 

```
COMMAND_NAME → IDENTIFIER
ARG_NAME → IDENTIFIER
ARG → 
| u8 (U8_INTEGER)
| u8 ([U8_INTEGER ( , U8_INTEGER )* ,?])
| u16(U16_INTEGER)
| u16 ([U16_INTEGER ( , U16_INTEGER )* ,?])
| u32(U32_INTEGER)
| u32 ([U32_INTEGER ( , U32_INTEGER )* ,?])
| u64(U64_INTEGER)
| u64 ([U64_INTEGER ( , U64_INTEGER )* ,?])
| i8 (I8_INTEGER)
| i8 ([I8_INTEGER ( , I8_INTEGER )* ,?])
| i16(I16_INTEGER)
| i16 ([I16_INTEGER ( , I16_INTEGER )* ,?])
| i32(I32_INTEGER)
| i32 ([I32_INTEGER ( , I32_INTEGER )* ,?])
| i64(I64_INTEGER)
| i64 ([I64_INTEGER ( , I64_INTEGER )* ,?])
| x8 (X8_INTEGER)
| x8 ([X8_INTEGER ( , X8_INTEGER )* ,?])
| x16(X16_INTEGER)
| x16 ([X16_INTEGER ( , X16_INTEGER )* ,?])
| x32(X32_INTEGER)
| x32 ([X32_INTEGER ( , X32_INTEGER )* ,?])
| x64(X64_INTEGER)
| x64 ([X64_INTEGER ( , X64_INTEGER )* ,?])
| f32(F32_INTEGER)
| f32 ([F32_INTEGER ( , F32_INTEGER )* ,?])
| f64(F64_INTEGER)
| f64 ([F64_INTEGER ( , F64_INTEGER )* ,?])
| bool(BOOLEAN)
| id(ENUM)
``` 

Use `0x..` notation for hexadecimal integers.

``` 
BOOLEAN      → "true" | "false"

ENUM         → ENUM_TYPE::ENUM_VAL
ENUM_TYPE    → IDENTIFIER
ENUM_VAL     → IDENTIFIER
``` 
