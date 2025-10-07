use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug, PartialEq)]
pub enum MemberType {
    X8,
    X16,
    X32,
    X64,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    ArrayOfX8,
    ArrayOfX16,
    ArrayOfX32,
    ArrayOfX64,
    ArrayOfU8,
    ArrayOfU16,
    ArrayOfU32,
    ArrayOfU64,
    ArrayOfI8,
    ArrayOfI16,
    ArrayOfI32,
    ArrayOfI64,
    ArrayOfF32,
    ArrayOfF64,
    Bool,
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub enum UniRecordArgVariant {
    X8(UniRecordArg<u8>),
    X16(UniRecordArg<u16>),
    X32(UniRecordArg<u32>),
    X64(UniRecordArg<u64>),
    U8(UniRecordArg<u8>),
    U16(UniRecordArg<u16>),
    U32(UniRecordArg<u32>),
    U64(UniRecordArg<u64>),
    I8(UniRecordArg<i8>),
    I16(UniRecordArg<i16>),
    I32(UniRecordArg<i32>),
    I64(UniRecordArg<i64>),
    F32(UniRecordArg<f32>),
    F64(UniRecordArg<f64>),
    ArrayOfX8(UniRecordArg<Vec<u8>>),
    ArrayOfX16(UniRecordArg<Vec<u16>>),
    ArrayOfX32(UniRecordArg<Vec<u32>>),
    ArrayOfX64(UniRecordArg<Vec<u64>>),
    ArrayOfU8(UniRecordArg<Vec<u8>>),
    ArrayOfU16(UniRecordArg<Vec<u16>>),
    ArrayOfU32(UniRecordArg<Vec<u32>>),
    ArrayOfU64(UniRecordArg<Vec<u64>>),
    ArrayOfI8(UniRecordArg<Vec<i8>>),
    ArrayOfI16(UniRecordArg<Vec<i16>>),
    ArrayOfI32(UniRecordArg<Vec<i32>>),
    ArrayOfI64(UniRecordArg<Vec<i64>>),
    ArrayOfF32(UniRecordArg<Vec<f32>>),
    ArrayOfF64(UniRecordArg<Vec<f64>>),
    Bool(UniRecordArg<bool>),
    Identifier(IdentifierRecordArg),
}

#[derive(Debug, PartialEq)]
pub struct IdentifierRecordArg{
    pub(crate) name: String,
    pub(crate) enum_type: String,
    pub(crate) value: String,
}

impl IdentifierRecordArg{

    pub fn dissassemble(self) ->(String, String,  String){
        (self.name, self.enum_type, self.value)
    }
}

#[derive(Debug, PartialEq)]
pub struct UniRecordArg<T> {
    pub(crate) name: String,
    pub(crate) value: T,
}
impl <T> UniRecordArg<T>{
    pub fn dissassemble(self) ->(String, T){
        (self.name, self.value)
    }
}

#[derive(Debug, PartialEq)]
pub enum RecordParsingError {
    BadX8Format(ParseIntError),
    BadX16Format(ParseIntError),
    BadX32Format(ParseIntError),
    BadX64Format(ParseIntError),
    BadU8Format(ParseIntError),
    BadU16Format(ParseIntError),
    BadU32Format(ParseIntError),
    BadU64Format(ParseIntError),
    BadI8Format(ParseIntError),
    BadI16Format(ParseIntError),
    BadI32Format(ParseIntError),
    BadI64Format(ParseIntError),
    BadF32Format(ParseFloatError),
    BadF64Format(ParseFloatError),
    BadX8ArrayFieldSyntax(ParseIntError, usize),
    BadX16ArrayFieldSyntax(ParseIntError, usize),
    BadX32ArrayFieldSyntax(ParseIntError, usize),
    BadX64ArrayFieldSyntax(ParseIntError, usize),
    BadU8ArrayFieldSyntax(ParseIntError, usize),
    BadU16ArrayFieldSyntax(ParseIntError, usize),
    BadU32ArrayFieldSyntax(ParseIntError, usize),
    BadU64ArrayFieldSyntax(ParseIntError, usize),
    BadI8ArrayFieldSyntax(ParseIntError, usize),
    BadI16ArrayFieldSyntax(ParseIntError, usize),
    BadI32ArrayFieldSyntax(ParseIntError, usize),
    BadI64ArrayFieldSyntax(ParseIntError, usize),
    BadF32ArrayFieldSyntax(ParseFloatError, usize),
    BadF64ArrayFieldSyntax(ParseFloatError, usize),
    BadBoolFormat(),
    BadIdFormat(),
}

impl Display for RecordParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadX8Format(e) => write!(f, "Error parsing u8 hexadecimal number: {e}"),
            Self::BadX8ArrayFieldSyntax(e, index) => write!(
                f,
                "Error parsing u8 hexadecimal number in array at pos {index} : {e}"
            ),
            Self::BadX16ArrayFieldSyntax(e, index) => write!(
                f,
                "Error parsing u16 hexadecimal number in array at pos {index}: {e}"
            ),
            Self::BadX32ArrayFieldSyntax(e, index) => write!(
                f,
                "Error parsing u32 hexadecimal number in array at pos {index}: {e}"
            ),
            Self::BadX64ArrayFieldSyntax(e, index) => write!(
                f,
                "Error parsing u64 hexadecimal number in array at pos {index}: {e}"
            ),
            _ => unimplemented!(),
        }
    }
}
impl UniRecordArgVariant {
    pub fn get_type(&self) -> MemberType{
        match self{
            UniRecordArgVariant::X8(_) => MemberType::X8,
            UniRecordArgVariant::X16(_) => MemberType::X16,
            UniRecordArgVariant::X32(_) => MemberType::X32,
            UniRecordArgVariant::X64(_) => MemberType::X64,
            UniRecordArgVariant::U8(_) => MemberType::U8,
            UniRecordArgVariant::U16(_) => MemberType::U16,
            UniRecordArgVariant::U32(_) => MemberType::U32,
            UniRecordArgVariant::U64(_) => MemberType::U64,
            UniRecordArgVariant::I8(_) => MemberType::I8,
            UniRecordArgVariant::I16(_) => MemberType::I16,
            UniRecordArgVariant::I32(_) => MemberType::I32,
            UniRecordArgVariant::I64(_) => MemberType::I64,
            UniRecordArgVariant::F32(_) => MemberType::F32,
            UniRecordArgVariant::F64(_) => MemberType::F64,
            UniRecordArgVariant::ArrayOfX8(_) => MemberType::ArrayOfX8,
            UniRecordArgVariant::ArrayOfX16(_) => MemberType::ArrayOfX16,
            UniRecordArgVariant::ArrayOfX32(_) => MemberType::ArrayOfX32,
            UniRecordArgVariant::ArrayOfX64(_) => MemberType::ArrayOfX64,
            UniRecordArgVariant::ArrayOfU8(_) => MemberType::ArrayOfU8,
            UniRecordArgVariant::ArrayOfU16(_) => MemberType::ArrayOfU16,
            UniRecordArgVariant::ArrayOfU32(_) => MemberType::ArrayOfU32,
            UniRecordArgVariant::ArrayOfU64(_) => MemberType::ArrayOfU64,
            UniRecordArgVariant::ArrayOfI8(_) => MemberType::ArrayOfI8,
            UniRecordArgVariant::ArrayOfI16(_) => MemberType::ArrayOfI16,
            UniRecordArgVariant::ArrayOfI32(_) => MemberType::ArrayOfI32,
            UniRecordArgVariant::ArrayOfI64(_) => MemberType::ArrayOfI64,
            UniRecordArgVariant::ArrayOfF32(_) => MemberType::ArrayOfF32,
            UniRecordArgVariant::ArrayOfF64(_) => MemberType::ArrayOfF64,
            UniRecordArgVariant::Identifier(arg) => MemberType::Identifier(arg.enum_type.clone()),
            UniRecordArgVariant::Bool(_) => MemberType::Bool,
        } 
    }

    pub fn get_name(&self) -> &String{
        match self{
            UniRecordArgVariant::X8(arg) => &arg.name,
            UniRecordArgVariant::X16(arg) => &arg.name,
            UniRecordArgVariant::X32(arg) => &arg.name,
            UniRecordArgVariant::X64(arg) => &arg.name,
            UniRecordArgVariant::U8(arg) => &arg.name,
            UniRecordArgVariant::U16(arg) => &arg.name,
            UniRecordArgVariant::U32(arg) => &arg.name,
            UniRecordArgVariant::U64(arg) => &arg.name,
            UniRecordArgVariant::I8(arg) => &arg.name,
            UniRecordArgVariant::I16(arg) => &arg.name,
            UniRecordArgVariant::I32(arg) => &arg.name,
            UniRecordArgVariant::I64(arg) => &arg.name,
            UniRecordArgVariant::F32(arg) => &arg.name,
            UniRecordArgVariant::F64(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfX8(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfX16(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfX32(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfX64(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfU8(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfU16(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfU32(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfU64(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfI8(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfI16(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfI32(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfI64(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfF32(arg) => &arg.name,
            UniRecordArgVariant::ArrayOfF64(arg) => &arg.name,
            UniRecordArgVariant::Identifier(arg) => &arg.name,
            UniRecordArgVariant::Bool(arg) => &arg.name,
        } 
    }

    pub fn from(
        record_arg_key: &str,
        record_arg_value: &str,
    ) -> Result<UniRecordArgVariant, RecordParsingError> {
        let arg_value_split = record_arg_value.split(&['(', ')']).collect::<Vec<&str>>();
        let arg_type = arg_value_split[0];
        let arg_value_content = arg_value_split[1];

        let record_arg_name = String::from(record_arg_key.split(':').next().unwrap());
        let value_field: Vec<&str> = arg_value_content.split(&['[', ']']).collect();

        // Array
        if value_field.len() > 1 {
            let value_list: Vec<&str> = value_field[1].split(',').collect();
            match arg_type {
                "x8" => {
                    return Ok(UniRecordArgVariant::ArrayOfX8(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(index, v)| {
                                let value = v.split('x').last().unwrap();
                                u8::from_str_radix(value, 16).map_err(|e| {
                                    RecordParsingError::BadX8ArrayFieldSyntax(e, index)
                                })
                            })
                            .collect::<Result<Vec<u8>, RecordParsingError>>()?,
                    }))
                }

                "x16" => {
                    return Ok(UniRecordArgVariant::ArrayOfX16(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(index, v)| {
                                let value = v.split('x').last().unwrap();
                                u16::from_str_radix(value, 16).map_err(|e| {
                                    RecordParsingError::BadX16ArrayFieldSyntax(e, index)
                                })
                            })
                            .collect::<Result<Vec<u16>, RecordParsingError>>()?,
                    }))
                }

                "x32" => {
                    return Ok(UniRecordArgVariant::ArrayOfX32(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(index, v)| {
                                let value = v.split('x').last().unwrap();
                                u32::from_str_radix(value, 16).map_err(|e| {
                                    RecordParsingError::BadX32ArrayFieldSyntax(e, index)
                                })
                            })
                            .collect::<Result<Vec<u32>, RecordParsingError>>()?,
                    }))
                }

                "x64" => {
                    return Ok(UniRecordArgVariant::ArrayOfX64(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(index, v)| {
                                let value = v.split('x').last().unwrap();
                                u64::from_str_radix(value, 16).map_err(|e| {
                                    RecordParsingError::BadX64ArrayFieldSyntax(e, index)
                                })
                            })
                            .collect::<Result<Vec<u64>, RecordParsingError>>()?,
                    }))
                }

                "u8" => {
                    return Ok(UniRecordArgVariant::ArrayOfU8(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadU8ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<u8>, RecordParsingError>>()?,
                    }))
                }
                "i8" => {
                    return Ok(UniRecordArgVariant::ArrayOfI8(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadI8ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<i8>, RecordParsingError>>()?,
                    }))
                }
                "u16" => {
                    return Ok(UniRecordArgVariant::ArrayOfU16(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadU16ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<u16>, RecordParsingError>>()?,
                    }))
                }
                "i16" => {
                    return Ok(UniRecordArgVariant::ArrayOfI16(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadI16ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<i16>, RecordParsingError>>()?,
                    }))
                }
                "u32" => {
                    return Ok(UniRecordArgVariant::ArrayOfU32(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadU32ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<u32>, RecordParsingError>>()?,
                    }))
                }
                "i32" => {
                    return Ok(UniRecordArgVariant::ArrayOfI32(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadI32ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<i32>, RecordParsingError>>()?,
                    }))
                }
                "u64" => {
                    return Ok(UniRecordArgVariant::ArrayOfU64(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadU64ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<u64>, RecordParsingError>>()?,
                    }))
                }
                "i64" => {
                    return Ok(UniRecordArgVariant::ArrayOfI64(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadI64ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<i64>, RecordParsingError>>()?,
                    }))
                }
                "f32" => {
                    return Ok(UniRecordArgVariant::ArrayOfF32(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadF32ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<f32>, RecordParsingError>>()?,
                    }))
                }
                "f64" => {
                    return Ok(UniRecordArgVariant::ArrayOfF64(UniRecordArg {
                        name: record_arg_name,
                        value: value_list
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                v.parse()
                                    .map_err(|e| RecordParsingError::BadF64ArrayFieldSyntax(e, i))
                            })
                            .collect::<Result<Vec<f64>, RecordParsingError>>()?,
                    }))
                }
                &_ => panic!("Record argument is an array of unknown type"),
            };
        } else {
            let value = value_field[0];
            match arg_type {
                "id" => {
                    let tokens: Vec<&str> = value.split("::").collect();
                    if tokens.len() != 2{
                        return Err(RecordParsingError::BadIdFormat()); 
                    }
                     
                    return Ok(UniRecordArgVariant::Identifier(IdentifierRecordArg {
                        name: record_arg_name,
                        enum_type: tokens[0].to_string(),
                        value: tokens[1].to_string(),
                    }));
                }
                "x8" => {
                    let value = value.split('x').last().unwrap();
                    let value = u8::from_str_radix(value, 16)
                        .map_err(|e| RecordParsingError::BadX8Format(e))?;

                    return Ok(UniRecordArgVariant::X8(UniRecordArg {
                        name: record_arg_name,
                        value,
                    }));
                }
                "x16" => {
                    let value = value.split('x').last().unwrap();
                    let value = u16::from_str_radix(value, 16)
                        .map_err(|e| RecordParsingError::BadX16Format(e))?;

                    return Ok(UniRecordArgVariant::X16(UniRecordArg {
                        name: record_arg_name,
                        value,
                    }));
                }
                "x32" => {
                    let value = value.split('x').last().unwrap();
                    let value = u32::from_str_radix(value, 16)
                        .map_err(|e| RecordParsingError::BadX32Format(e))?;

                    return Ok(UniRecordArgVariant::X32(UniRecordArg {
                        name: record_arg_name,
                        value,
                    }));
                }
                "x64" => {
                    let value = value.split('x').last().unwrap();
                    let value = u64::from_str_radix(value, 16)
                        .map_err(|e| RecordParsingError::BadX64Format(e))?;

                    return Ok(UniRecordArgVariant::X64(UniRecordArg {
                        name: record_arg_name,
                        value,
                    }));
                }

                "u8" => {
                    return Ok(UniRecordArgVariant::U8(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadU8Format(e))?,
                    }))
                }
                "i8" => {
                    return Ok(UniRecordArgVariant::I8(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadI8Format(e))?,
                    }))
                }
                "u16" => {
                    return Ok(UniRecordArgVariant::U16(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadU16Format(e))?,
                    }))
                }
                "i16" => {
                    return Ok(UniRecordArgVariant::I16(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadI16Format(e))?,
                    }))
                }
                "u32" => {
                    return Ok(UniRecordArgVariant::U32(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadU32Format(e))?,
                    }))
                }
                "i32" => {
                    return Ok(UniRecordArgVariant::I32(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadI32Format(e))?,
                    }))
                }
                "u64" => {
                    return Ok(UniRecordArgVariant::U64(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadU64Format(e))?,
                    }))
                }
                "i64" => {
                    return Ok(UniRecordArgVariant::I64(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadI64Format(e))?,
                    }))
                }
                "f32" => {
                    return Ok(UniRecordArgVariant::F32(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadF32Format(e))?,
                    }))
                }
                "f64" => {
                    return Ok(UniRecordArgVariant::F64(UniRecordArg {
                        name: record_arg_name,
                        value: value
                            .parse()
                            .map_err(|e| RecordParsingError::BadF64Format(e))?,
                    }))
                }
                "bool" => {
                    let bool_value;
                    if let Ok(value) = value.parse::<bool>() {
                        bool_value = value;
                    } else {
                        let value = value.split('x').last().unwrap();
                        let value = u64::from_str_radix(value, 16)
                            .map_err(|_| RecordParsingError::BadBoolFormat())?;
                        bool_value = value != 0;
                    }
                    return Ok(UniRecordArgVariant::Bool(UniRecordArg {
                        name: record_arg_name,
                        value: bool_value,
                    }));
                }

                &_ => panic!("Unhandled record argument type !"),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UniRecord {
    name: String,
    args: Vec<UniRecordArgVariant>,
}

impl UniRecord {
    pub fn new(name: String, args: Vec<UniRecordArgVariant>) -> UniRecord {
        UniRecord { name, args }
    }

    pub fn name(&self) -> &String{
        return &self.name;
    }


    pub fn args(&self) -> &Vec<UniRecordArgVariant>{
        return &self.args;
    }

    pub fn dissassemble(self) ->(String, Vec<UniRecordArgVariant>){
        (self.name, self.args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn record_arg_parsing_test() {
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "x8(0x25)"),
            Ok(UniRecordArgVariant::X8(UniRecordArg {
                name: String::from("test_val"),
                value: 0x25
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "x16(0xFF3A)"),
            Ok(UniRecordArgVariant::X16(UniRecordArg {
                name: String::from("test_val"),
                value: 0xFF3A
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "x32(0x23A4F89E)"),
            Ok(UniRecordArgVariant::X32(UniRecordArg {
                name: String::from("test_val"),
                value: 0x23A4F89E
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "x64(0xEE3FB4FAD554CE05)"),
            Ok(UniRecordArgVariant::X64(UniRecordArg {
                name: String::from("test_val"),
                value: 0xEE3FB4FAD554CE05
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "u8(250)"),
            Ok(UniRecordArgVariant::U8(UniRecordArg {
                name: String::from("test_val"),
                value: 250
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "u16(25690)"),
            Ok(UniRecordArgVariant::U16(UniRecordArg {
                name: String::from("test_val"),
                value: 25690
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "u32(589214450)"),
            Ok(UniRecordArgVariant::U32(UniRecordArg {
                name: String::from("test_val"),
                value: 589214450
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "u64(56228558925412450)"),
            Ok(UniRecordArgVariant::U64(UniRecordArg {
                name: String::from("test_val"),
                value: 56228558925412450
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "i8(-105)"),
            Ok(UniRecordArgVariant::I8(UniRecordArg {
                name: String::from("test_val"),
                value: -105
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "i16(-5240)"),
            Ok(UniRecordArgVariant::I16(UniRecordArg {
                name: String::from("test_val"),
                value: -5240
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "i32(-405251740)"),
            Ok(UniRecordArgVariant::I32(UniRecordArg {
                name: String::from("test_val"),
                value: -405251740
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "i64(-85405652565187840)"),
            Ok(UniRecordArgVariant::I64(UniRecordArg {
                name: String::from("test_val"),
                value: -85405652565187840
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "f32(-40525.1740)"),
            Ok(UniRecordArgVariant::F32(UniRecordArg {
                name: String::from("test_val"),
                value: -40525.1740
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test-val:", "f64(-8540565256.5187840)"),
            Ok(UniRecordArgVariant::F64(UniRecordArg {
                name: String::from("test-val"),
                value: -8540565256.5187840
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "x8([0x25])"),
            Ok(UniRecordArgVariant::ArrayOfX8(UniRecordArg {
                name: String::from("test_val"),
                value: vec![0x25]
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "x8([0x2A, 0x6F])"),
            Ok(UniRecordArgVariant::ArrayOfX8(UniRecordArg {
                name: String::from("test_val"),
                value: vec![0x2A, 0x6F]
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "u8([88])"),
            Ok(UniRecordArgVariant::ArrayOfU8(UniRecordArg {
                name: String::from("test_val"),
                value: vec![88]
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "u8([88,65])"),
            Ok(UniRecordArgVariant::ArrayOfU8(UniRecordArg {
                name: String::from("test_val"),
                value: vec![88, 65]
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "f32([52.6])"),
            Ok(UniRecordArgVariant::ArrayOfF32(UniRecordArg {
                name: String::from("test_val"),
                value: vec![52.6]
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "f32([52.6,-42.4])"),
            Ok(UniRecordArgVariant::ArrayOfF32(UniRecordArg {
                name: String::from("test_val"),
                value: vec![52.6, -42.4]
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "bool(false)"),
            Ok(UniRecordArgVariant::Bool(UniRecordArg {
                name: String::from("test_val"),
                value: false
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "bool(52)"),
            Ok(UniRecordArgVariant::Bool(UniRecordArg {
                name: String::from("test_val"),
                value: true
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "bool(0)"),
            Ok(UniRecordArgVariant::Bool(UniRecordArg {
                name: String::from("test_val"),
                value: false
            }))
        );
        assert_eq!(
            UniRecordArgVariant::from("test_val:", "id(EnumType::Hello)"),
            Ok(UniRecordArgVariant::Identifier(IdentifierRecordArg {
                name: String::from("test_val"),
                enum_type: String::from("EnumType"),
                value: String::from("Hello")
            }))
        );
    }
}
