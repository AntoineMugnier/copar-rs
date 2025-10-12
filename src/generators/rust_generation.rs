use crate::{
    code_generation_commons::generate_blank_line, // if you have this helper for writing blank lines
    model::{ArrayInstanceVariant, OperationParameterVariant},
    unirecord::MemberType,
    Model,
};

mod private {
    use super::*;
    use crate::unirecord::MemberType;

    pub trait Sealed {
        fn generate_rust_file(&mut self, output_file: &mut impl std::io::Write);
        fn generate_rust_operation_id_enum(&mut self, output_file: &mut impl std::io::Write);
        fn generate_rust_operation_variant_enum(&mut self, output_file: &mut impl std::io::Write);
        fn generate_rust_enums(&mut self, output_file: &mut impl std::io::Write);
        fn generate_rust_structs(&mut self, output_file: &mut impl std::io::Write);
        fn generate_rust_arrays(&mut self, output_file: &mut impl std::io::Write);
        fn generate_rust_instances(&mut self, output_file: &mut impl std::io::Write);
        fn generate_rust_operation_list(&mut self, output_file: &mut impl std::io::Write);
        fn member_type_to_rust_type_string(member_type: &MemberType) -> String;
        fn fmt_rust_array_value<T, F: Fn(&T) -> String>(array: &[T], format_function: F) -> String;
        fn fmt_rust_array_instance(
            array_instance_variant: &ArrayInstanceVariant,
            array_name: &str,
        ) -> String;
        fn fmt_rust_struct_member(
            operation_parameter_variant: &OperationParameterVariant,
        ) -> String;
        // Helpers
    }
}

use private::Sealed;
use stringcase::{macro_case, pascal_case, snake_case};

/// Trait allowing model to generate Rust code
pub trait RustGeneration: private::Sealed {
    fn compute_to_rust(&mut self, output_rust_file: &mut impl std::io::Write);
}

impl RustGeneration for Model {
    fn compute_to_rust(&mut self, output_rust_file: &mut impl std::io::Write) {
        self.generate_rust_file(output_rust_file);
    }
}

impl private::Sealed for Model {
    fn generate_rust_file(&mut self, output_file: &mut impl std::io::Write) {
        self.generate_rust_operation_id_enum(output_file);
        self.generate_rust_operation_variant_enum(output_file);

        self.generate_rust_enums(output_file);
        self.generate_rust_structs(output_file);

        self.generate_rust_arrays(output_file);
        self.generate_rust_instances(output_file);
        self.generate_rust_operation_list(output_file);
    }

    fn generate_rust_operation_id_enum(&mut self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]").unwrap();
        writeln!(output_file, "pub enum OperationId {{").unwrap();
        for record_name in self.defined_records.keys() {
            writeln!(output_file, "    {},", pascal_case(record_name)).unwrap();
        }
        writeln!(output_file, "}}").unwrap();
        generate_blank_line(output_file);
    }

    fn generate_rust_operation_variant_enum(&mut self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "#[derive(Debug, Clone)]").unwrap();
        writeln!(output_file, "pub enum OperationVariant<'a> {{").unwrap();
        for struct_name in self.defined_records.keys() {
            writeln!(
                output_file,
                "    {}(&'a {}),",
                pascal_case(struct_name),
                pascal_case(struct_name)
            )
            .unwrap();
        }
        writeln!(output_file, "}}").unwrap();
        generate_blank_line(output_file);
    }

    fn generate_rust_enums(&mut self, output_file: &mut impl std::io::Write) {
        for (enum_type_name, enum_members) in self.defined_enums.iter() {
            writeln!(output_file, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]").unwrap();
            writeln!(output_file, "pub enum {} {{", pascal_case(enum_type_name)).unwrap();
            for enum_member in enum_members.iter() {
                writeln!(output_file, "    {},", pascal_case(enum_member)).unwrap();
            }
            writeln!(output_file, "}}").unwrap();
            generate_blank_line(output_file);
        }
    }

    fn generate_rust_structs(&mut self, output_file: &mut impl std::io::Write) {
        for (struct_name, struct_members) in self.defined_records.iter() {
            writeln!(output_file, "#[derive(Debug, Clone, PartialEq)]").unwrap();
            writeln!(output_file, "pub struct {} {{", pascal_case(struct_name)).unwrap();
            for struct_member in struct_members {
                // in Rust fields are snake_case, types are mapped to Rust types
                let field_name = snake_case(&struct_member.member_name);
                let rust_type = Self::member_type_to_rust_type_string(&struct_member.member_type);
                writeln!(output_file, "    pub {}: {},", field_name, rust_type).unwrap();
            }
            writeln!(output_file, "}}").unwrap();
            generate_blank_line(output_file);
        }
    }

    fn generate_rust_arrays(&mut self, output_file: &mut impl std::io::Write) {
        for (array_variant, array_instance_name) in self.instanciated_arrays.iter() {
            // array_instance_name -> snake_case for Rust static name
            let rust_array_name = macro_case(array_instance_name);
            writeln!(
                output_file,
                "{}",
                Self::fmt_rust_array_instance(array_variant, &rust_array_name),
            )
            .unwrap();
        }
        if !self.instanciated_arrays.is_empty() {
            generate_blank_line(output_file);
        }
    }

    fn generate_rust_instances(&mut self, output_file: &mut impl std::io::Write) {
        for (operation, operation_instance_name) in self.operation_instances.iter() {
            let operation_type = pascal_case(&operation.operation_type);
            let rust_instance_name = macro_case(operation_instance_name);
            write!(
                output_file,
                "pub static {}: {} = {} {{ ",
                rust_instance_name, operation_type, operation_type
            )
            .unwrap();

            let nb_parameters = operation.parameters.len();

            for (index, operation_parameter) in operation.parameters.iter().enumerate() {
                // convert field name to snake_case and format the value
                let param_assignment = Self::fmt_rust_struct_member(operation_parameter);
                write!(output_file, "{}", param_assignment).unwrap();
                if index < nb_parameters - 1 {
                    write!(output_file, ", ").unwrap();
                }
            }
            writeln!(output_file, " }};").unwrap();
        }

        if !self.operation_instances.is_empty() {
            generate_blank_line(output_file);
        }
    }

    fn generate_rust_operation_list(&mut self, output_file: &mut impl std::io::Write) {
        let array_name = macro_case(self.sequence_name.as_ref().unwrap());

        writeln!(
            output_file,
            "pub static {}: &[OperationVariant] = &[",
            array_name
        )
        .unwrap();

        // We will emit a sequence of Operation values; referencing static instances by name.
        for op_ref in self.operation_ref_table.iter() {
            let operation_id_member = pascal_case(&op_ref.operation_type);
            let rust_instance_name = macro_case(&op_ref.operation_variant_ref_name);

            writeln!(
                output_file,
                "    OperationVariant::{}(&{}),",
                operation_id_member, rust_instance_name
            )
            .unwrap();
        }

        writeln!(output_file, "];").unwrap();
    }

    fn member_type_to_rust_type_string(member_type: &MemberType) -> String {
        match member_type {
            MemberType::X8 | MemberType::U8 => "u8".to_string(),
            MemberType::X16 | MemberType::U16 => "u16".to_string(),
            MemberType::X32 | MemberType::U32 => "u32".to_string(),
            MemberType::X64 | MemberType::U64 => "u64".to_string(),
            MemberType::I8 => "i8".to_string(),
            MemberType::I16 => "i16".to_string(),
            MemberType::I32 => "i32".to_string(),
            MemberType::I64 => "i64".to_string(),
            MemberType::F32 => "f32".to_string(),
            MemberType::F64 => "f64".to_string(),
            MemberType::ArrayOfX8 | MemberType::ArrayOfU8 => "&'static [u8]".to_string(),
            MemberType::ArrayOfX16 | MemberType::ArrayOfU16 => "&'static [u16]".to_string(),
            MemberType::ArrayOfX32 | MemberType::ArrayOfU32 => "&'static [u32]".to_string(),
            MemberType::ArrayOfX64 | MemberType::ArrayOfU64 => "&'static [u64]".to_string(),
            MemberType::ArrayOfI8 => "&'static [i8]".to_string(),
            MemberType::ArrayOfI16 => "&'static [i16]".to_string(),
            MemberType::ArrayOfI32 => "&'static [i32]".to_string(),
            MemberType::ArrayOfI64 => "&'static [i64]".to_string(),
            MemberType::ArrayOfF32 => "&'static [f32]".to_string(),
            MemberType::ArrayOfF64 => "&'static [f64]".to_string(),
            MemberType::Bool => "bool".to_string(),
            MemberType::Identifier(enum_type) => pascal_case(enum_type),
        }
    }

    fn fmt_rust_array_value<T, F: Fn(&T) -> String>(array: &[T], format_function: F) -> String {
        let mut ret = String::from("[");
        let nb_elements = array.len();
        for (index, element) in array.iter().enumerate() {
            ret += &format_function(element);
            if index < nb_elements - 1 {
                ret += ", ";
            }
        }
        ret += "]";
        ret
    }

    fn fmt_rust_array_instance(
        array_instance_variant: &ArrayInstanceVariant,
        array_name: &str, // snake_case
    ) -> String {
        let (array_value, rust_type) = match array_instance_variant {
            ArrayInstanceVariant::X8(array) => (
                Self::fmt_rust_array_value(array, |e| format!("0x{:x}", e)),
                "u8",
            ),
            ArrayInstanceVariant::U8(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "u8",
            ),
            ArrayInstanceVariant::X16(array) => (
                Self::fmt_rust_array_value(array, |e| format!("0x{:x}", e)),
                "u16",
            ),
            ArrayInstanceVariant::U16(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "u16",
            ),
            ArrayInstanceVariant::X32(array) => (
                Self::fmt_rust_array_value(array, |e| format!("0x{:x}", e)),
                "u32",
            ),
            ArrayInstanceVariant::U32(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "u32",
            ),
            ArrayInstanceVariant::X64(array) => (
                Self::fmt_rust_array_value(array, |e| format!("0x{:x}", e)),
                "u64",
            ),
            ArrayInstanceVariant::U64(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "u64",
            ),
            ArrayInstanceVariant::I8(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "i8",
            ),
            ArrayInstanceVariant::I16(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "i16",
            ),
            ArrayInstanceVariant::I32(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "i32",
            ),
            ArrayInstanceVariant::I64(array) => (
                Self::fmt_rust_array_value(array, |e| format!("{}", e)),
                "i64",
            ),
            _ => {
                unimplemented!(
                    "Array type in ArrayInstanceVariant not supported for Rust generation"
                )
            }
        };

        format!(
            "pub static {}: & [{}] = &{};",
            array_name, rust_type, array_value
        )
    }

    fn fmt_rust_struct_member(operation_parameter_variant: &OperationParameterVariant) -> String {
        // For struct literal assignment we emit "field_name: value"
        match operation_parameter_variant {
            OperationParameterVariant::X8(param) => {
                let name = snake_case(&param.name);
                format!("{}: 0x{:x}", name, param.value)
            }
            OperationParameterVariant::U8(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::I8(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::X16(param) => {
                let name = snake_case(&param.name);
                format!("{}: 0x{:x}", name, param.value)
            }
            OperationParameterVariant::U16(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::I16(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::X32(param) => {
                let name = snake_case(&param.name);
                format!("{}: 0x{:x}", name, param.value)
            }
            OperationParameterVariant::U32(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::I32(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::X64(param) => {
                let name = snake_case(&param.name);
                format!("{}: 0x{:x}", name, param.value)
            }
            OperationParameterVariant::U64(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::I64(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::F32(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::F64(param) => {
                let name = snake_case(&param.name);
                format!("{}: {}", name, param.value)
            }
            OperationParameterVariant::ArrayOfX8(param)
            | OperationParameterVariant::ArrayOfU8(param)
            | OperationParameterVariant::ArrayOfI8(param)
            | OperationParameterVariant::ArrayOfX16(param)
            | OperationParameterVariant::ArrayOfU16(param)
            | OperationParameterVariant::ArrayOfI16(param)
            | OperationParameterVariant::ArrayOfX32(param)
            | OperationParameterVariant::ArrayOfU32(param)
            | OperationParameterVariant::ArrayOfI32(param)
            | OperationParameterVariant::ArrayOfX64(param)
            | OperationParameterVariant::ArrayOfU64(param)
            | OperationParameterVariant::ArrayOfI64(param)
            | OperationParameterVariant::ArrayOfF32(param)
            | OperationParameterVariant::ArrayOfF64(param) => {
                let name = snake_case(&param.name);
                let rust_array_name = macro_case(&param.value);
                format!("{}: {}", name, rust_array_name)
            }
            OperationParameterVariant::Bool(param) => {
                let name = snake_case(&param.name);
                let val_str = if param.value { "true" } else { "false" };
                format!("{}: {}", name, val_str)
            }
            OperationParameterVariant::Identifier(param) => {
                // enum type and value assumed PascalCase
                let name = snake_case(&param.name);
                let enum_type = pascal_case(&param.enum_type);
                let enum_value = pascal_case(&param.value);
                format!("{}: {}::{}", name, enum_type, enum_value)
            }
        }
    }
}
