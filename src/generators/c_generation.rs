use super::generate_blank_line;
use crate::{
    model::{ArrayInstanceVariant, OperationParameterVariant},
    unirecord::MemberType,
    Model,
};
use stringcase::{macro_case, snake_case};

mod private {
    use super::*;
    use crate::unirecord::MemberType;

    pub trait Sealed {
        fn generate_c_header(&self, output_file: &mut impl std::io::Write);
        fn generate_header_pre(&self, output_file: &mut impl std::io::Write);
        fn generate_header_post(&self, output_file: &mut impl std::io::Write);
        fn generate_operation_id_enum(&self, output_file: &mut impl std::io::Write);
        fn generate_operation_variants_definition(&self, output_file: &mut impl std::io::Write);
        fn generate_operation_definition(&self, output_file: &mut impl std::io::Write);
        fn generate_enum_definitions(&self, output_file: &mut impl std::io::Write);
        fn generate_struct_definitions(&self, output_file: &mut impl std::io::Write);
        fn generate_operation_list(&self, output_file: &mut impl std::io::Write);
        fn generate_array_instances(&self, output_file: &mut impl std::io::Write);
        fn generate_operation_instances(&self, output_file: &mut impl std::io::Write);
        fn generate_source_pre(&self, output_file: &mut impl std::io::Write);
        fn generate_source_post(&self, output_file: &mut impl std::io::Write);
        fn generate_c_source(&self, output_file: &mut impl std::io::Write);
        fn member_type_to_c_type_string(member_type: &MemberType) -> String;
        fn fmt_c_array_value<T, F: Fn(&T) -> String>(array: &[T], format_function: F) -> String;
        fn fmt_array_instance(
            array_instance_variant: &ArrayInstanceVariant,
            array_name: &str,
        ) -> String;
        fn fmt_struct_member(operation_parameter_variant: &OperationParameterVariant) -> String;
    }
}

use private::Sealed;

/// Trait allowing copar model to generate C code
pub trait CGeneration: private::Sealed {
    /// Generate C code from the model
    fn compute_to_c(
        &self,
        output_c_file: &mut impl std::io::Write,
        output_h_file: &mut impl std::io::Write,
    );
}

impl CGeneration for Model {
    fn compute_to_c(
        &self,
        output_c_file: &mut impl std::io::Write,
        output_h_file: &mut impl std::io::Write,
    ) {
        self.generate_c_header(output_h_file);
        self.generate_c_source(output_c_file);
    }
}

impl private::Sealed for Model {
    fn generate_header_pre(&self, output_file: &mut impl std::io::Write) {
        let sequence_name = self.sequence_name.as_ref().unwrap().to_uppercase();
        let sequence_name = sequence_name.as_str();
        write!(output_file, "#ifndef _{sequence_name}_H\n#define _{sequence_name}_H\n#include <stdint.h>\n#include <stdbool.h>\n").unwrap();
    }

    fn generate_header_post(&self, output_file: &mut impl std::io::Write) {
        write!(output_file, "#endif").unwrap();
    }

    fn generate_operation_id_enum(&self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "enum OperationId{{").unwrap();

        for (record_name, _record_members) in self.defined_records.iter() {
            let record_name = macro_case(record_name);
            writeln!(output_file, "   OPERATION_ID_{},", record_name).unwrap();
        }
        writeln!(output_file, "}};").unwrap();
    }

    fn generate_operation_variants_definition(&self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "union OperationVariant{{").unwrap();
        for (struct_name, _) in self.defined_records.iter() {
            let struct_member_variant = snake_case(struct_name);
            writeln!(
                output_file,
                "   const {struct_name}* const {struct_member_variant};"
            )
            .unwrap();
        }
        writeln!(output_file, "}};").unwrap();
    }

    fn generate_operation_definition(&self, output_file: &mut impl std::io::Write) {
        write!(output_file, "typedef struct{{\n   const enum OperationId id;\n   const union OperationVariant variant;\n}}Operation;\n").unwrap();
    }

    fn generate_enum_definitions(&self, output_file: &mut impl std::io::Write) {
        for (enum_type_name, enum_members) in self.defined_enums.iter() {
            writeln!(output_file, "enum {enum_type_name}{{").unwrap();
            for enum_member in enum_members.iter() {
                writeln!(output_file, "   {enum_type_name}{enum_member},").unwrap();
            }
            writeln!(output_file, "}};").unwrap();
        }
    }

    fn generate_struct_definitions(&self, output_file: &mut impl std::io::Write) {
        for (struct_name, struct_members) in self.defined_records.iter() {
            writeln!(output_file, "typedef struct{{").unwrap();
            for struct_member in struct_members {
                let struct_member_name = struct_member.member_name.as_str();
                let struct_member_type =
                    Self::member_type_to_c_type_string(&struct_member.member_type);
                writeln!(output_file, "   {struct_member_type} {struct_member_name};").unwrap();
            }
            writeln!(output_file, "}}{struct_name};",).unwrap();
            generate_blank_line(output_file);
        }
    }

    fn generate_operation_list(&self, output_file: &mut impl std::io::Write) {
        writeln!(
            output_file,
            "const Operation {}[] = {{",
            self.sequence_name.as_ref().unwrap()
        )
        .unwrap();

        let nb_operations = self.operation_ref_table.len();
        for (index, operation_table_member) in self.operation_ref_table.iter().enumerate() {
            let record_name = macro_case(&operation_table_member.operation_type);
            let operation_id = format!("OPERATION_ID_{}", record_name);
            let operation_variant_instance_ref =
                operation_table_member.operation_variant_ref_name.as_str();
            let operation_variant_name = snake_case(&operation_table_member.operation_type);
            write!(output_file, "   {{.id = {operation_id}, .variant={{.{operation_variant_name}=&{operation_variant_instance_ref}}}}}",).unwrap();
            if index < nb_operations - 1 {
                writeln!(output_file, ",").unwrap();
            }
        }

        write!(output_file, "\n}};\n",).unwrap();
        generate_blank_line(output_file);
    }

    fn generate_array_instances(&self, output_file: &mut impl std::io::Write) {
        for (operation, operation_instance_name) in self.instanciated_arrays.iter() {
            writeln!(
                output_file,
                "{}",
                Self::fmt_array_instance(operation, operation_instance_name),
            )
            .unwrap();
        }
        generate_blank_line(output_file);
    }

    fn generate_operation_instances(&self, output_file: &mut impl std::io::Write) {
        for (operation, operation_instance_name) in self.operation_instances.iter() {
            let operation_type = operation.operation_type.as_str();
            write!(
                output_file,
                "const {operation_type} {operation_instance_name} = {{"
            )
            .unwrap();
            let nb_parameters = operation.parameters.len();

            for (index, operation_parameter) in operation.parameters.iter().enumerate() {
                let parameter_name = Self::fmt_struct_member(operation_parameter);
                write!(output_file, "{}", parameter_name).unwrap();
                if index < nb_parameters - 1 {
                    write!(output_file, ", ").unwrap();
                }
            }

            writeln!(output_file, "}};").unwrap();
        }
        generate_blank_line(output_file);
    }

    fn generate_source_pre(&self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "#include \"playdisc.h\"").unwrap();
        generate_blank_line(output_file);
    }

    fn generate_source_post(&self, output_file: &mut impl std::io::Write) {
        let sequence_name = self.sequence_name.as_ref().unwrap().to_lowercase();
        write!(
            output_file,
            "const uint32_t {}_len = sizeof({}_operations)/sizeof(Operation);",
            sequence_name, sequence_name
        )
        .unwrap();
    }

    fn generate_c_source(&self, output_file: &mut impl std::io::Write) {
        self.generate_source_pre(output_file);
        self.generate_array_instances(output_file);
        self.generate_operation_instances(output_file);
        self.generate_operation_list(output_file);
        self.generate_source_post(output_file);
    }

    fn generate_c_header(&self, output_file: &mut impl std::io::Write) {
        self.generate_header_pre(output_file);
        generate_blank_line(output_file);
        self.generate_operation_id_enum(output_file);
        generate_blank_line(output_file);
        self.generate_enum_definitions(output_file);
        generate_blank_line(output_file);
        self.generate_struct_definitions(output_file);
        self.generate_operation_variants_definition(output_file);
        generate_blank_line(output_file);
        self.generate_operation_definition(output_file);
        generate_blank_line(output_file);
        self.generate_header_post(output_file);
    }

    fn member_type_to_c_type_string(member_type: &MemberType) -> String {
        let ret = match member_type {
            MemberType::X8 => "uint8_t",
            MemberType::X16 => "uint16_t",
            MemberType::X32 => "uint32_t",
            MemberType::X64 => "uint64_t",
            MemberType::U8 => "uint8_t",
            MemberType::U16 => "uint8_t",
            MemberType::U32 => "uint32_t",
            MemberType::U64 => "uint64_t",
            MemberType::I8 => "int8_t",
            MemberType::I16 => "int16_t",
            MemberType::I32 => "int32_t",
            MemberType::I64 => "int64_t",
            MemberType::F32 => "float",
            MemberType::F64 => "double",
            MemberType::ArrayOfX8 => "uint8_t* const",
            MemberType::ArrayOfX16 => "uint16_t* const",
            MemberType::ArrayOfX32 => "uint32_t* const",
            MemberType::ArrayOfX64 => "uint64_t* const",
            MemberType::ArrayOfU8 => "uint8_t* const",
            MemberType::ArrayOfU16 => "uint16_t* const",
            MemberType::ArrayOfU32 => "uint32_t* const",
            MemberType::ArrayOfU64 => "uint64_t* const",
            MemberType::ArrayOfI8 => "int8_t* const",
            MemberType::ArrayOfI16 => "int16_t* const",
            MemberType::ArrayOfI32 => "int32_t* const",
            MemberType::ArrayOfI64 => "int64_t* const",
            MemberType::ArrayOfF32 => "float* const",
            MemberType::ArrayOfF64 => "double* const",
            MemberType::Bool => "bool",
            MemberType::Identifier(enum_type) => {
                return "const enum ".to_string() + enum_type.as_str()
            }
        };
        String::from("const ") + ret
    }

    fn fmt_c_array_value<T, F: Fn(&T) -> String>(array: &[T], format_function: F) -> String {
        let mut ret;
        let nb_elements = array.len();
        ret = String::from("{");
        for (index, element) in array.iter().enumerate() {
            ret += format_function(element).as_str();

            if index < nb_elements - 1 {
                ret += ", ";
            }
        }
        ret += "}";
        ret
    }

    fn fmt_array_instance(
        array_instance_variant: &ArrayInstanceVariant,
        array_name: &str,
    ) -> String {
        let array_value;
        let array_type;
        match array_instance_variant {
            ArrayInstanceVariant::X8(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("0x{:x}", element));
                array_type = "uint8_t";
            }

            ArrayInstanceVariant::X16(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("0x{:x}", element));
                array_type = "uint16_t";
            }

            ArrayInstanceVariant::X32(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("0x{:x}", element));
                array_type = "uint32_t";
            }
            ArrayInstanceVariant::X64(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("0x{:x}", element));
                array_type = "uint64_t";
            }
            ArrayInstanceVariant::U8(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "uint8_t";
            }
            ArrayInstanceVariant::U16(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "uint16_t";
            }
            ArrayInstanceVariant::U32(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "uint32_t";
            }
            ArrayInstanceVariant::U64(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "uint64_t";
            }
            ArrayInstanceVariant::I8(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "int8_t";
            }
            ArrayInstanceVariant::I16(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "int16_t";
            }
            ArrayInstanceVariant::I32(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "int32_t";
            }
            ArrayInstanceVariant::I64(array) => {
                array_value = Self::fmt_c_array_value(array, |element| format!("{}", element));
                array_type = "int64_t";
            }
            _ => unimplemented!(),
        }
        format!("const {} {}[] = {};", array_type, array_name, array_value)
    }

    fn fmt_struct_member(operation_parameter_variant: &OperationParameterVariant) -> String {
        match operation_parameter_variant {
            OperationParameterVariant::X8(param) => {
                format!(".{} = 0x{:x}", param.name, param.value)
            }
            OperationParameterVariant::X16(param) => {
                format!(".{} = 0x{:x}", param.name, param.value)
            }
            OperationParameterVariant::X32(param) => {
                format!(".{} = 0x{:x}", param.name, param.value)
            }
            OperationParameterVariant::X64(param) => {
                format!(".{} = 0x{:x}", param.name, param.value)
            }
            OperationParameterVariant::U8(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::U16(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::U32(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::U64(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::I8(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::I16(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::I32(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::I64(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::F32(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::F64(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfX8(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfX16(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfX32(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfX64(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfU8(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfU16(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfU32(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfU64(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfI8(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfI16(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfI32(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfI64(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfF32(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::ArrayOfF64(param) => {
                format!(".{} = {}", param.name, param.value)
            }
            OperationParameterVariant::Bool(param) => {
                let val = if param.value {
                    "true".to_string()
                } else {
                    "false".to_string()
                };
                format!(".{} = {}", param.name, val)
            }
            OperationParameterVariant::Identifier(param) => {
                let val = param.enum_type.clone() + param.value.as_str();
                format!(".{} = {}", param.name, val)
            }
        }
    }
}
