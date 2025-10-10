use crate::unirecord::{IdentifierRecordArg, MemberType, UniRecord, UniRecordArgVariant};
use indexmap::IndexMap;
use std::hash::Hash;

use ordered_float::OrderedFloat;
type OrderedF32 = OrderedFloat<f32>;
type OrderedF64 = OrderedFloat<f64>;

#[derive(Debug)]
pub(crate) struct StructureDefinitionMember {
    pub(crate) member_name: String,
    pub(crate) member_type: MemberType,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct OperationParameter<T> {
    pub(crate) name: String,
    pub(crate) value: T,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IdentifierOperationParameter {
    pub(crate) name: String,
    pub(crate) enum_type: String,
    pub(crate) value: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OperationParameterVariant {
    X8(OperationParameter<u8>),
    X16(OperationParameter<u16>),
    X32(OperationParameter<u32>),
    X64(OperationParameter<u64>),
    U8(OperationParameter<u8>),
    U16(OperationParameter<u16>),
    U32(OperationParameter<u32>),
    U64(OperationParameter<u64>),
    I8(OperationParameter<i8>),
    I16(OperationParameter<i16>),
    I32(OperationParameter<i32>),
    I64(OperationParameter<i64>),
    F32(OperationParameter<OrderedF32>),
    F64(OperationParameter<OrderedF64>),
    ArrayOfX8(OperationParameter<String>),
    ArrayOfX16(OperationParameter<String>),
    ArrayOfX32(OperationParameter<String>),
    ArrayOfX64(OperationParameter<String>),
    ArrayOfU8(OperationParameter<String>),
    ArrayOfU16(OperationParameter<String>),
    ArrayOfU32(OperationParameter<String>),
    ArrayOfU64(OperationParameter<String>),
    ArrayOfI8(OperationParameter<String>),
    ArrayOfI16(OperationParameter<String>),
    ArrayOfI32(OperationParameter<String>),
    ArrayOfI64(OperationParameter<String>),
    ArrayOfF32(OperationParameter<String>),
    ArrayOfF64(OperationParameter<String>),
    Bool(OperationParameter<bool>),
    Identifier(IdentifierOperationParameter),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct Operation {
    pub(crate) operation_type: String,
    pub(crate) parameters: Vec<OperationParameterVariant>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ArrayInstanceVariant {
    X8(Vec<u8>),
    X16(Vec<u16>),
    X32(Vec<u32>),
    X64(Vec<u64>),
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    F32(Vec<OrderedF32>),
    F64(Vec<OrderedF64>),
}

#[derive(Debug)]
pub(crate) struct OperationTableMember {
    pub(crate) operation_type: String,
    pub(crate) operation_variant_ref_name: String,
}

#[derive(Debug)]
pub struct Model {
    pub(crate) sequence_name: Option<String>,
    pub(crate) array_instance_counter: usize,
    pub(crate) operation_instance_counter: usize,
    pub(crate) defined_enums: IndexMap<String, Vec<String>>,
    pub(crate) defined_records: IndexMap<String, Vec<StructureDefinitionMember>>,
    pub(crate) instanciated_arrays: IndexMap<ArrayInstanceVariant, String>,
    pub(crate) operation_instances: IndexMap<Operation, String>,
    pub(crate) operation_ref_table: Vec<OperationTableMember>,
}

impl Model {
    pub fn set_sequence_name(&mut self, sequence_name: String) {
        self.sequence_name = Some(sequence_name);
    }

    fn add_identifier_declaration(&mut self, argument: &IdentifierRecordArg) {
        let enum_type = argument.enum_type.clone();
        let enum_value = argument.value.clone();

        // If no declaration exists for this enum, create it
        if let Some(enum_decl) = self.defined_enums.get_mut(&enum_type) {
            if !enum_decl.contains(&enum_value) {
                enum_decl.push(enum_value);
            }
        } else {
            let new_enum_decl = vec![enum_value];
            self.defined_enums.insert(enum_type, new_enum_decl);
        }
    }

    fn add_array_instance(&mut self, array_variant: ArrayInstanceVariant) -> String {
        if let Some(array_instance_name) = self.instanciated_arrays.get_mut(&array_variant) {
            array_instance_name.clone()
        } else {
            let new_array_instance_name =
                String::from("array_") + self.array_instance_counter.to_string().as_str();
            self.array_instance_counter += 1;
            self.instanciated_arrays
                .insert(array_variant, new_array_instance_name.clone());
            new_array_instance_name
        }
    }
    pub fn add_structure_definition(
        &mut self,
        record_name: String,
        record_args: &[UniRecordArgVariant],
    ) {
        // If no declaration exists for this record, create it
        if self.defined_records.get(&record_name).is_none() {
            let mut defined_structure_members = Vec::new();
            for record_arg in record_args.iter() {
                let member = StructureDefinitionMember {
                    member_name: record_arg.get_name().clone(),
                    member_type: record_arg.get_type(),
                };
                defined_structure_members.push(member);
            }
            self.defined_records
                .insert(record_name, defined_structure_members);
        }
    }

    pub fn add_record(&mut self, record: UniRecord) {
        let (record_type, record_args) = record.dissassemble();
        self.add_structure_definition(record_type.clone(), &record_args);

        let mut operation_parameters = Vec::new();

        for arg in record_args {
            let parameter = match arg {
                UniRecordArgVariant::X8(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::X8(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::X16(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::X16(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::X32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::X32(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::X64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::X64(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::U8(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::U8(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::U16(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::U16(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::U32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::U32(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::U64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::U64(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::I8(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::I8(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::I16(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::I16(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::I32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::I32(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::I64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::I64(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::F32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::F32(OperationParameter {
                        name: arg_name,
                        value: OrderedF32::from(arg_value),
                    })
                }
                UniRecordArgVariant::F64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::F64(OperationParameter {
                        name: arg_name,
                        value: OrderedF64::from(arg_value),
                    })
                }
                UniRecordArgVariant::ArrayOfX8(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::X8(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfX8(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }
                UniRecordArgVariant::ArrayOfX16(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::X16(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfX16(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }
                UniRecordArgVariant::ArrayOfX32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::X32(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfX32(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }

                UniRecordArgVariant::ArrayOfX64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::X64(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfX64(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }

                UniRecordArgVariant::ArrayOfU8(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::U8(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfU8(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }
                UniRecordArgVariant::ArrayOfU16(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::U16(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfU16(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }
                UniRecordArgVariant::ArrayOfU32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::U32(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfU32(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }

                UniRecordArgVariant::ArrayOfU64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::U64(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfU64(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }

                UniRecordArgVariant::ArrayOfI8(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::I8(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfI8(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }
                UniRecordArgVariant::ArrayOfI16(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::I16(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfI16(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }
                UniRecordArgVariant::ArrayOfI32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::I32(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfI32(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }

                UniRecordArgVariant::ArrayOfI64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let array_instance = ArrayInstanceVariant::I64(arg_value);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfI64(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }

                UniRecordArgVariant::ArrayOfF32(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let f32_array: Vec<OrderedF32> =
                        arg_value.iter().map(|x| OrderedF32::from(*x)).collect();
                    let array_instance = ArrayInstanceVariant::F32(f32_array);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfF32(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }

                UniRecordArgVariant::ArrayOfF64(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();
                    let f64_array: Vec<OrderedF64> =
                        arg_value.iter().map(|x| OrderedF64::from(*x)).collect();
                    let array_instance = ArrayInstanceVariant::F64(f64_array);
                    let parameter_value = self.add_array_instance(array_instance);
                    OperationParameterVariant::ArrayOfF64(OperationParameter {
                        name: arg_name,
                        value: parameter_value,
                    })
                }
                UniRecordArgVariant::Bool(arg) => {
                    let (arg_name, arg_value) = arg.dissassemble();

                    OperationParameterVariant::Bool(OperationParameter {
                        name: arg_name,
                        value: arg_value,
                    })
                }
                UniRecordArgVariant::Identifier(arg) => {
                    self.add_identifier_declaration(&arg);
                    let (enum_name, enum_type, enum_value) = arg.dissassemble();

                    OperationParameterVariant::Identifier(IdentifierOperationParameter {
                        name: enum_name,
                        enum_type,
                        value: enum_value,
                    })
                }
            };
            operation_parameters.push(parameter);
        }

        let operation = Operation {
            operation_type: record_type.clone(),
            parameters: operation_parameters,
        };

        let operation_instance_name;
        if let Some(operation_instance_name_) = self.operation_instances.get_mut(&operation) {
            operation_instance_name = operation_instance_name_.clone();
        } else {
            operation_instance_name =
                String::from("operation_") + self.operation_instance_counter.to_string().as_str();
            self.operation_instance_counter += 1;
            self.operation_instances
                .insert(operation, operation_instance_name.clone());
        }

        let operation_table_member = OperationTableMember {
            operation_type: record_type,
            operation_variant_ref_name: operation_instance_name,
        };

        self.operation_ref_table.push(operation_table_member);
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            sequence_name: None,
            array_instance_counter: 0,
            operation_instance_counter: 0,
            defined_enums: IndexMap::new(),
            defined_records: IndexMap::new(),
            instanciated_arrays: IndexMap::new(),
            operation_instances: IndexMap::new(),
            operation_ref_table: Vec::new(),
        }
    }
}
